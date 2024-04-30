use std::rc::Rc;

use crate::tokenizer::{Keyword, Token, Tokenizer};

pub struct Parser {
    tokenizer: Tokenizer,
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            tokens: vec![],
            index: 0,
        }
    }

    pub fn parse(&mut self, input: Rc<str>) -> Token {
        self.tokenizer.init(input);

        self.tokens = self.tokenizer.tokenize();

        match &self.tokens[self.index] {
            Token::Word(w) => match w.keyword {
                Keyword::Select => return self.parse_query(),
                _ => {}
            },
            _ => {}
        }
        // add better handling

        unimplemented!()
    }

    // scan token
    fn scan() {
        unimplemented!()
    }

    // Parse a SELECT Query
    fn parse_query(&mut self) -> Token {
        // self.tokens[self.index]
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn it_works() {
        let result = Parser::new().parse("select 1".into());

        println!("{:?}", result);
    }
}
