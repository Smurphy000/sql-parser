use std::rc::Rc;

use crate::tokenizer::{Keyword, Token, Tokenizer};

#[derive(Debug)]
pub enum Statement {
    Select(Select),
}

#[derive(Debug)]
pub enum SelectItem {
    UnnamedExpr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Identifier(Ident),
} // TODO this should be an ENUM since there are many different types of Expressions

#[derive(Debug)]
pub struct Ident {
    value: String,
}

#[derive(Debug)]
pub struct Table {
    // TODO this currently does not support Qualified table names separated by '.'
    name: String,
    // TODO
    // alias
}

impl Table {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

// TODO need to add a display repr and a validator function
#[derive(Debug)]
pub struct Select {
    selection: Option<Expr>,     // filter like in a where clause
    projection: Vec<SelectItem>, // columns being selected
    from: Table,                 // table being queried
}

impl Select {
    pub fn new() -> Self {
        Self {
            selection: None,
            projection: vec![],
            from: Table::new("".into()), // TODO table name should be overwritable
        }
    }
}

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

    pub fn parse(&mut self, input: Rc<str>) -> Statement {
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
    fn parse_query(&mut self) -> Statement {
        // RDP
        // projection := "SELECT" <column_list>
        // column_list := <column> <column_list>
        // column := expr
        // expr := column_name
        //
        // selection := "WHERE" <filter_exp>
        // filter_exp :=
        //
        // table := "FROM" <table_ident>
        // table_ident :=  <ident>
        let mut query = Select::new();
        let mut it = self.tokens.iter().peekable();

        while let Some(&token) = it.peek() {
            match token {
                Token::Word(w) => {
                    match w.keyword {
                        Keyword::Select => {
                            // parse until end of SELECT, currently include column names, separated by comma tokens
                            // progress index
                            // query.projection = vec![SelectItem::UnnamedExpr()];
                        }
                        Keyword::From => {
                            // parse until end of FROM, this should be a FROM token followed by a table name
                            // progress index
                        }
                        Keyword::Where => {
                            // parse until end of WHERE, this should be an expression (which can more 1 or more expression)
                            // progress index
                        }
                        _ => {
                            // unsupported token
                        }
                    }
                }
                _ => {
                    //error state, first token must by a Word / Keyword
                }
            }
        }

        Statement::Select(query)
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
