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
    keyword: Keyword,
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
// For inspiration: https://doc.rust-lang.org/nightly/nightly-rustc/src/rustc_lexer/lib.rs.html
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

    fn is_whitespace(c: char) -> bool {
        unimplemented!()
    }

    // This should take in a generic function which can eat characters of the same type
    fn eat_while(&self) {}

    // TODO can probably make eat generic similar to rustc lexer, where it take a comparitor function for continuing
    fn eat(&self, it: &mut Peekable<Chars<'_>>) -> String {
        let mut ident = String::new();

        while let Some(&c) = it.peek() {
            match c {
                c if c.is_alphabetic() => {
                    ident.push(c);
                    it.next();
                }
                // no longer an alphabetic character, such as a space
                _ => return ident,
            }
        }
        ident
    }

    fn eat_number(&self, it: &mut Peekable<Chars<'_>>) -> String {
        let mut num = String::new();

        while let Some(&c) = it.peek() {
            match c {
                c if c.is_numeric() => {
                    num.push(c);
                    it.next();
                }
                // no longer an alphabetic character, such as a space
                ' ' => return num,
                _ => {} // this is the error case
            }
        }
        num
    }

    // fully processes input in one call, or make this return an iterator
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut it = self.input.chars().peekable();

        while let Some(&c) = it.peek() {
            match c {
                c if c.is_alphabetic() => {
                    // TODO words should be forced into same casing for matching purposes
                    let word = self.eat(&mut it).to_uppercase();
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
                    it.next();
                }
                c if c.is_numeric() => {
                    let number = self.eat_number(&mut it);
                    tokens.push(Token::Number(number.parse::<i64>().unwrap()));
                    it.next();
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
                ' ' => {
                    // TODO consume whitespace
                    it.next();
                }
                _ => {
                    // TODO need to handle unknown tokens, or raise an error
                }
            }
        }

        tokens
    }

    // This function returns an iterator of Tokens, allowing the caller to stream tokens
    // instead of getting a vec
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
    pub fn next_iter(&mut self) -> Token {
        let mut token = Token::EOF;
        let mut it = self.input.chars().peekable();
        if self.cursor > 0 {
            it.nth(self.cursor as usize);
        }
        if self.cursor > self.input.len() as u32 {
            return Token::EOF;
        }

        while let Some(&c) = it.peek() {
            match c {
                c if c.is_alphabetic() => {
                    // TODO words should be forced into same casing for matching purposes
                    let word = self.eat(&mut it).to_uppercase();
                    self.cursor += word.len() as u32;
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
                    let number = self.eat_number(&mut it);
                    self.cursor += number.len() as u32;
                    token = Token::Number(number.parse::<i64>().unwrap());
                    return token;
                }
                '*' => {
                    token = Token::Asterisk;
                    self.cursor += 1;
                }
                '-' => {
                    token = Token::Minus;
                    self.cursor += 1;
                }
                '+' => {
                    token = Token::Plus;
                    self.cursor += 1;
                }
                '(' => {
                    token = Token::LeftParen;
                    self.cursor += 1;
                }
                ')' => {
                    token = Token::RightParen;
                    self.cursor += 1;
                }
                ' ' => {
                    // TODO consume whitespace
                    it.next();
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
    use crate::*;

    #[test]
    fn it_works() {
        let mut tokenizer = Tokenizer::new();
        tokenizer.init("select )(+-*".into());
        let result = tokenizer.tokenize();
        println!("{:?}", result);

        tokenizer.init("1234 + 4567".into());
        let result = tokenizer.tokenize();
        println!("{:?}", result);

        tokenizer.init("select somecol + 1 from sometable".into());
        let result = tokenizer.tokenize();
        println!("{:?}", result);
    }

    #[test]
    fn iterable_tokenizer() {
        let mut tokenizer = Tokenizer::new();
        tokenizer.init("abc 123".into());
        let result = tokenizer.tokenize_iter();

        let vec_result: Vec<Token> = result.collect();
        println!("{:?}", vec_result);
    }
}
