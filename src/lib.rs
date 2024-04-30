use std::{fmt, io::Error, rc::Rc};
mod parser;
mod tokenizer;
use crate::parser::Parser;
use crate::tokenizer::{Keyword, Token, Tokenizer};

#[derive(Debug)]
pub struct Query {
    _type: String,
    body: Token,
}

impl Query {
    fn new(body: Token) -> Self {
        Self {
            _type: String::from("Query"),
            body: body,
        }
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type: {}, body: {:?}", self._type, self.body)
    }
}

// Parser should be pub since it is the exposed API to parse SQL
// pub struct Parser {
//     tokenizer: Tokenizer,
//     look_ahead: Option<Token>,
// }

// impl Parser {
//     pub fn new() -> Self {
//         Self {
//             tokenizer: Tokenizer::new(),
//             look_ahead: None,
//         }
//     }

//     pub fn parse(&mut self, input: &str) {
//         unimplemented!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {}
// }
