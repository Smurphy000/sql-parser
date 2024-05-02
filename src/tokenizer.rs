use std::{iter::Peekable, rc::Rc, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Select,
    From,
    Where,
    Group,
    Order,
    Having,
    NoKeyword,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Word {
    value: String,
    pub keyword: Keyword,
}

impl Word {
    pub fn new(value: String, keyword: Keyword) -> Self {
        Self { value, keyword }
    }
}

// TODO token may be better as a struct, have a TokenType attribute
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Word(Word),
    Number(i64),
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Asterisk,
    Comma,
    Whitespace,
    EOF,
}

pub struct Tokenizer {
    input: Rc<str>,
    cursor: u32,
}

/// Tokenizer for SQL Parser
///
/// inspiration: <https://doc.rust-lang.org/nightly/nightly-rustc/src/rustc_lexer/lib.rs.html>
///
/// ```rust
/// # use sql_parser::tokenizer::*;
/// let mut tokenizer = Tokenizer::new();
/// tokenizer.init("test string".into());
/// let tokens = tokenizer.tokenize();
/// assert_eq!(tokens, vec![
///     Token::Word(Word::new("TEST".into(), Keyword::NoKeyword)),
///     Token::Whitespace,
///     Token::Word(Word::new("STRING".into(), Keyword::NoKeyword)),
///     Token::EOF
/// ]);
/// ```
impl Tokenizer {
    pub fn new() -> Self {
        Self {
            input: "".into(),
            cursor: 0,
        }
    }

    pub fn init(&mut self, input: Rc<str>) {
        self.input = input;
        self.cursor = 0;
    }

    // This function should take in a generic function which can eat characters of the same type.
    // It also progresses to the input to the character following the String / token
    fn eat_while(&self, it: &mut Peekable<Chars<'_>>, f: fn(char) -> bool) -> String {
        let mut ident = String::new();
        while let Some(&c) = it.peek() {
            match c {
                c if f(c) => {
                    ident.push(c);
                    it.next();
                }
                _ => return ident,
            }
        }
        ident
    }

    /// Consumes Tokenizers input and fully processes into a vector of tokens
    ///
    /// ```rust
    ///  # use sql_parser::tokenizer::*;
    /// # let mut tokenizer = Tokenizer::new();
    /// # tokenizer.init("test string".into());
    /// let tokens = tokenizer.tokenize();
    /// assert_eq!(tokens, vec![
    ///     Token::Word(Word::new("TEST".into(), Keyword::NoKeyword)),
    ///     Token::Whitespace,
    ///     Token::Word(Word::new("STRING".into(), Keyword::NoKeyword)),
    ///     Token::EOF
    /// ]);
    /// ```
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut it = self.input.chars().peekable();

        while let Some(&c) = it.peek() {
            match c {
                c if c.is_alphabetic() => {
                    let word = self
                        .eat_while(&mut it, |c| {
                            if c.is_alphanumeric() || c == '_' {
                                return true;
                            }
                            false
                        })
                        .to_uppercase();
                    match word.as_str() {
                        // TODO add remaining match arms for keywords
                        "SELECT" => tokens.push(Token::Word(Word::new(word, Keyword::Select))),
                        "FROM" => tokens.push(Token::Word(Word::new(word, Keyword::From))),
                        "WHERE" => tokens.push(Token::Word(Word::new(word, Keyword::Where))),
                        "HAVING" => tokens.push(Token::Word(Word::new(word, Keyword::Having))),
                        // Placeholder for Group By and Order By, will need to consume the following word "By"
                        // "FROM" => tokens.push(Token::Word(Word::new(word, Keyword::Group))),
                        // "FROM" => tokens.push(Token::Word(Word::new(word, Keyword::Order))),
                        _ => tokens.push(Token::Word(Word::new(word, Keyword::NoKeyword))),
                    }
                }
                c if c.is_numeric() => {
                    let number = self.eat_while(&mut it, |c| {
                        if c.is_numeric() {
                            return true;
                        }
                        false
                    });
                    tokens.push(Token::Number(number.parse::<i64>().unwrap()));
                }
                '*' => {
                    tokens.push(Token::Asterisk);
                    it.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    it.next();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    it.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    it.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    it.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    it.next();
                }
                ' ' => {
                    let _ = self.eat_while(&mut it, |c| {
                        if c == ' ' {
                            return true;
                        }

                        false
                    });
                    tokens.push(Token::Whitespace);
                }
                _ => {
                    // TODO need to handle unknown tokens, or raise an error
                }
            }
        }

        // TODO should we actually return an EOF token, is it needed?
        // Finished consuming String, add an EOF
        tokens.push(Token::EOF);
        tokens
    }

    /// tokenize_iter returns an iterator of Tokens, allowing the caller to stream tokens
    /// instead of getting a vec
    ///
    /// this function does not end with an EOF token
    ///
    /// ```rust
    /// # use sql_parser::tokenizer::*;
    /// # let mut tokenizer = Tokenizer::new();
    /// # tokenizer.init("test string".into());
    /// let tokens: Vec<Token> = tokenizer.tokenize_iter().collect();
    /// assert_eq!(tokens, vec![
    ///     Token::Word(Word::new("TEST".into(), Keyword::NoKeyword)),
    ///     Token::Whitespace,
    ///     Token::Word(Word::new("STRING".into(), Keyword::NoKeyword)),
    /// ]);
    /// ```
    pub fn tokenize_iter(&mut self) -> impl Iterator<Item = Token> + '_ {
        std::iter::from_fn(move || {
            let token = self.next_iter();
            if token != Token::EOF {
                Some(token)
            } else {
                None
            }
        })
    }

    // This function returns the next token, progressing the cursor
    // Currently progresses cursor using `nth` and adding the length of the token created to the cursor
    fn next_iter(&mut self) -> Token {
        let mut token = Token::EOF;
        let mut it = self.input.chars().peekable();
        if self.cursor > 0 {
            it.nth((self.cursor - 1) as usize);
        }

        if self.cursor >= self.input.len() as u32 {
            return Token::EOF;
        }

        while let Some(&c) = it.peek() {
            match c {
                c if c.is_alphabetic() => {
                    let word = self
                        .eat_while(&mut it, |c| {
                            if c.is_alphanumeric() || c == '_' {
                                return true;
                            }
                            false
                        })
                        .to_uppercase();
                    self.cursor += (word.len()) as u32;
                    match &word.as_str() {
                        // TODO add remaining match arms for keywords
                        &"SELECT" => token = Token::Word(Word::new(word, Keyword::Select)),
                        &"FROM" => token = Token::Word(Word::new(word, Keyword::From)),
                        &"WHERE" => token = Token::Word(Word::new(word, Keyword::Where)),
                        &"HAVING" => token = Token::Word(Word::new(word, Keyword::Having)),
                        // Placeholder for Group By and Order By, will need to consume the following word "By"
                        // "FROM" => tokens.push(Token::Word(Word::new(word, Keyword::Group))),
                        // "FROM" => tokens.push(Token::Word(Word::new(word, Keyword::Order))),
                        _ => token = Token::Word(Word::new(word, Keyword::NoKeyword)),
                    }
                    return token;
                }
                c if c.is_numeric() => {
                    let number = self.eat_while(&mut it, |c| {
                        if c.is_numeric() {
                            return true;
                        }
                        false
                    });
                    self.cursor += (number.len()) as u32;
                    token = Token::Number(number.parse::<i64>().unwrap());
                    return token;
                }
                '*' => {
                    token = Token::Asterisk;
                    self.cursor += 1;
                    return token;
                }
                '-' => {
                    token = Token::Minus;
                    self.cursor += 1;
                    return token;
                }
                '+' => {
                    token = Token::Plus;
                    self.cursor += 1;
                    return token;
                }
                '(' => {
                    token = Token::LeftParen;
                    self.cursor += 1;
                    return token;
                }
                ')' => {
                    token = Token::RightParen;
                    self.cursor += 1;
                    return token;
                }
                ',' => {
                    token = Token::Comma;
                    self.cursor += 1;
                    return token;
                }
                ' ' => {
                    let space = self.eat_while(&mut it, |c| {
                        if c == ' ' {
                            return true;
                        }

                        false
                    });
                    token = Token::Whitespace;
                    self.cursor += space.len() as u32;
                    return token;
                }
                _ => {
                    // TODO need to handle unknown tokens, or raise an error
                }
            }
        }

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_token_types() {
        let mut tokenizer = Tokenizer::new();
        tokenizer.init("select  )(+-*".into());
        let result = tokenizer.tokenize();
        assert_eq!(
            result,
            vec![
                Token::Word(Word::new("SELECT".into(), Keyword::Select)),
                Token::Whitespace,
                Token::RightParen,
                Token::LeftParen,
                Token::Plus,
                Token::Minus,
                Token::Asterisk,
                Token::EOF
            ]
        );
    }

    #[test]
    fn equation() {
        let mut tokenizer = Tokenizer::new();
        tokenizer.init("1234 + 4567".into());
        let result = tokenizer.tokenize();

        assert_eq!(
            result,
            vec![
                Token::Number(1234),
                Token::Whitespace,
                Token::Plus,
                Token::Whitespace,
                Token::Number(4567),
                Token::EOF
            ]
        );
    }

    #[test]
    fn query_like() {
        let mut tokenizer = Tokenizer::new();
        tokenizer.init("select somecol + 1, col_2 from sometable".into());
        let result = tokenizer.tokenize();

        assert_eq!(
            result,
            vec![
                Token::Word(Word::new("SELECT".into(), Keyword::Select)),
                Token::Whitespace,
                Token::Word(Word::new("SOMECOL".into(), Keyword::NoKeyword)),
                Token::Whitespace,
                Token::Plus,
                Token::Whitespace,
                Token::Number(1),
                Token::Comma,
                Token::Whitespace,
                Token::Word(Word::new("COL_2".into(), Keyword::NoKeyword)),
                Token::Whitespace,
                Token::Word(Word::new("FROM".into(), Keyword::From)),
                Token::Whitespace,
                Token::Word(Word::new("SOMETABLE".into(), Keyword::NoKeyword)),
                Token::EOF
            ]
        );
    }

    #[test]
    fn iterable_tokenizer() {
        let mut tokenizer = Tokenizer::new();
        tokenizer.init("abc 123".into());
        let result = tokenizer.tokenize_iter();

        let vec_result: Vec<Token> = result.collect();
        assert_eq!(
            vec_result,
            vec![
                Token::Word(Word::new("ABC".into(), Keyword::NoKeyword)),
                Token::Whitespace,
                Token::Number(123)
            ]
        );
    }
}
