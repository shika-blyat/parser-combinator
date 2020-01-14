use crate::combinators::{many, many1};
use crate::common::take_whitespaces;
use crate::error::ParserError;
use crate::math::{into_postfix, take_number, take_operator};

pub type Parser<T, X> = Box<dyn Fn(X) -> Result<(String, T), ParserError>>;
pub type Expr = Vec<Atom>;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    I32(i32),
    F32(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U32,
    I32,
    F32,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Num(Number),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Lit(Literal),
    Var(String),
    Op(Operator),
    Parens(Expr),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    pub lexeme: String,
    pub precedence: i32,
    pub assoc: Assoc,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Assoc {
    Left,
    Right,
    Both,
}

pub fn take_expr() -> Parser<Expr, String> {
    Box::new(move |s| {
        let (remaining, num) = take_number()(s)?;
        let (remaining, _) = take_whitespaces()(remaining)?;
        let mut expr: Vec<Atom> = vec![Atom::Lit(Literal::Num(num))];
        let (remaining, values) = many(Box::new(|s| {
            let mut temp_expr = (Atom::Var("".to_string()), Atom::Var("".to_string()));
            take_operator()(s)
                .and_then(|(remaining, op)| {
                    temp_expr.0 = op;
                    take_whitespaces()(remaining)
                })
                .and_then(|(remaining, _)| {
                    let (remaining, num) = take_number()(remaining)?;
                    temp_expr.1 = Atom::Lit(Literal::Num(num));
                    let (remaining, _) = take_whitespaces()(remaining)?;
                    Ok((remaining, temp_expr))
                })
        }))(remaining)?;
        for i in values {
            expr.push(i.0);
            expr.push(i.1);
        }
        Ok((remaining, expr))
    })
}
