use crate::combinators::{many, many1};
use crate::common::{take_char, take_whitespaces};
use crate::error::ParserError;
use crate::math::{into_ast, take_number, take_operator};

pub type Parser<T, X> = Box<dyn Fn(X) -> Result<(String, T), ParserError>>;

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
pub enum OpTerm {
    Op(Operator),
    OpTerm(Expr),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Lit(Literal),
    Var(String),
    Operation(Vec<OpTerm>),
    BinOp(Box<Bin>),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    pub lexeme: String,
    pub precedence: i32,
    pub assoc: Assoc,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Assoc {
    Left,
    Right,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Bin {
    Binary {
        left: Expr,
        op: Operator,
        right: Expr,
    },
    Expr(Expr),
}
impl Bin {
    pub fn new_bin(left: Expr, op: Operator, right: Expr) -> Self {
        Self::Binary { left, op, right }
    }
    pub fn new_expr(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}
pub fn take_expr() -> Parser<Vec<OpTerm>, String> {
    Box::new(move |s| {
        let (remaining, mut expr) = take_number()(s)
            .and_then(|(remaining, expr)| {
                Ok((
                    remaining,
                    vec![OpTerm::OpTerm(Expr::Lit(Literal::Num(expr)))],
                ))
            })
            .or_else(|error| {
                take_char('(')(error.remaining()).and_then(|(remaining, _)| {
                    let (remaining, expr) = many(take_expr())(remaining)?;
                    let (remaining, _) = take_char(')')(remaining)?;
                    Ok((
                        remaining,
                        vec![OpTerm::OpTerm(Expr::Operation(expr[0].clone()))],
                    ))
                })
            })?;
        let (remaining, _) = take_whitespaces()(remaining)?;
        let (remaining, values) = many(Box::new(|s| {
            let mut temp_expr = vec![
                OpTerm::OpTerm(Expr::Var("".to_string())),
                OpTerm::OpTerm(Expr::Var("".to_string())),
            ];
            take_operator()(s)
                .and_then(|(remaining, op)| {
                    temp_expr[0] = op;
                    take_whitespaces()(remaining)
                })
                .and_then(|(remaining, _)| {
                    let (remaining, num) = take_number()(remaining)?;
                    temp_expr[1] = OpTerm::OpTerm(Expr::Lit(Literal::Num(num)));
                    let (remaining, _) = take_whitespaces()(remaining)?;
                    Ok((remaining, temp_expr))
                })
                .or_else(|error| {
                    take_char('(')(error.remaining()).and_then(|(remaining, _)| {
                        let (remaining, expr) = many(take_expr())(remaining)?;
                        let (remaining, _) = take_char(')')(remaining)?;
                        Ok((
                            remaining,
                            vec![OpTerm::OpTerm(Expr::Operation(expr[0].clone()))],
                        ))
                    })
                })
        }))(remaining)?;
        for i in values {
            for j in i {
                expr.push(j);
            }
        }
        Ok((remaining, expr))
    })
}
pub fn build_ast() -> Parser<Bin, String> {
    Box::new(|s| {
        take_expr()(s).and_then(|(remaining, tokens)| Ok((remaining, into_ast()(tokens)?.1)))
    })
}
