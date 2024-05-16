use std::{fmt, iter::Peekable, path::Display, rc::Rc};

use crate::tokenizer::{Keyword, Token, Tokenizer};

#[derive(Debug)]
pub enum Statement {
    Select(Select),
    None,
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

impl Ident {
    pub fn new(value: String) -> Self {
        Self { value }
    }
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
#[derive(Debug, Default)]
pub struct Select {
    selection: Option<Expr>,     // filter like in a where clause
    projection: Vec<SelectItem>, // columns being selected
    from: Option<Table>,         // table being queried
}

impl Select {
    pub fn new(selection: Option<Expr>, projection: Vec<SelectItem>, from: Option<Table>) -> Self {
        Self {
            selection: selection,
            projection: projection,
            from: from,
        }
    }
}

// impl fmt::Display for Select {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         unimplemented!()
//     }
// }

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
        println!("{:?}", &self.tokens);
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

    fn parse_projection<'a>(&self, it: &mut Peekable<std::slice::Iter<Token>>) -> Vec<SelectItem> {
        let mut items: Vec<SelectItem> = vec![];

        while let Some(&t) = it.peek() {
            match t {
                Token::Word(w) => {
                    // If we reach the end of SELECT
                    if w.keyword == Keyword::From {
                        return items;
                    }
                    items.push(SelectItem::UnnamedExpr(Expr::Identifier(Ident::new(
                        w.value.clone(),
                    ))));
                    it.next();
                }
                Token::Number(n) => {
                    items.push(SelectItem::UnnamedExpr(Expr::Identifier(Ident::new(
                        n.to_string(),
                    ))));
                    it.next();
                }
                Token::Comma => {
                    // reset current accumulation of tokens that produce some expression
                }
                Token::Whitespace => {
                    it.next();
                }

                _ => return items,
            }
        }
        items
    }

    fn parse_from(&self, it: &mut Peekable<std::slice::Iter<Token>>) -> Option<Table> {
        let mut table = Table::new("".into());
        while let Some(&t) = it.peek() {
            match t {
                Token::Word(w) => {
                    // If we reach the end of SELECT
                    if w.keyword == Keyword::From {
                        it.next();
                        continue;
                    }

                    table.name = w.value.clone();
                    it.next();
                    return Some(table);
                }

                Token::Whitespace => {
                    it.next();
                }

                _ => return None,
            }
        }
        None
    }

    fn parse_filter(&self, it: &mut Peekable<std::slice::Iter<Token>>) -> Option<Expr> {
        None
    }

    fn parse_select<'a>(&self, it: &mut Peekable<std::slice::Iter<Token>>) -> Select {
        it.next();
        let projection = self.parse_projection(it);
        let from = self.parse_from(it);
        let filter = self.parse_filter(it);
        Select::new(filter, projection, from)
    }

    // Parse a SELECT Query
    fn parse_query(&mut self) -> Statement {
        // RDP
        // projection := "SELECT" <column_list>
        // column_list := <column> | <column> <sep> <column_list>
        // column := expr
        // expr := ident | ...
        // sep := , | ...
        //
        // selection := "WHERE" <filter_exp>
        // filter_exp :=
        //
        // table := "FROM" <table_ident>
        // table_ident :=  <ident>
        let mut it = self.tokens.iter().peekable();

        while let Some(&token) = it.peek() {
            match token {
                Token::Word(w) => {
                    match w.keyword {
                        Keyword::Select => {
                            // parse until end of SELECT, currently include column names, separated by comma tokens
                            // progress index
                            return Statement::Select(self.parse_select(&mut it));
                        }
                        _ => {
                            // unsupported token
                            it.next();
                        }
                    }
                }
                _ => {
                    //error state, first token must by a Word / Keyword
                    it.next();
                }
            }
        }
        Statement::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Parser::new().parse("select hi from table_1 where 1 = 1".into());

        println!("{:?}", result);
    }
}
