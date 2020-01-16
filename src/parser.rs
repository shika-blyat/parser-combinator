use crate::combinators::{many, many1};
use crate::common::{take_char, take_whitespaces};
use crate::error::ParserError;
use crate::math::{into_ast, take_number, take_operator};

pub type Parser<T, X> = Box<dyn Fn(X) -> Result<(String, T), ParserError>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    I32(i32),
    F32(f32),
    //U32(u32),
}
impl Number {
    pub fn get_type(&self) -> Type {
        match self {
            Self::I32(_) => Type::I32,
            //Self::U32(_) => Type::U32,
            Self::F32(_) => Type::F32,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Type {
    U32,
    I32,
    F32,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Num(Number),
}
impl Literal {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Num(num) => num.get_type(),
        }
    }
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
impl Expr {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Lit(literal) => literal.get_type(),
            Self::Var(_) => panic!("Cannot know the type of a variable yet"),
            _ => unreachable!(), // In the typed ast, there is normally no Operation variant
        }
    }
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
    Bin(Binary),
    Uno(Expr),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Expr,
    pub op: Operator,
    pub right: Expr,
    pub expr_type: Option<Type>,
}
impl Bin {
    pub fn new_bin(left: Expr, op: Operator, right: Expr) -> Self {
        Self::Bin(Binary {
            left,
            op,
            right,
            expr_type: None,
        })
    }
    pub fn new_bin_typed(left: Expr, op: Operator, right: Expr, expr_type: Type) -> Self {
        Self::Bin(Binary {
            left,
            op,
            right,
            expr_type: Some(expr_type),
        })
    }
    pub fn new_uno(expr: Expr) -> Self {
        Self::Uno(expr)
    }
    pub fn get_type(&self) -> Type {
        match self {
            Bin::Bin(binary) => binary.expr_type.clone().unwrap(),
            Bin::Uno(expr) => expr.get_type(),
        }
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
                    let (remaining, expr) = many1(take_expr())(remaining)?;
                    let (remaining, _) = take_char(')')(remaining)?;
                    Ok((
                        remaining,
                        vec![OpTerm::OpTerm(Expr::Operation(
                            expr.into_iter().nth(0).unwrap(),
                        ))],
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
                        let (remaining, expr) = many1(take_expr())(remaining)?;
                        let (remaining, _) = take_char(')')(remaining)?;
                        Ok((
                            remaining,
                            vec![OpTerm::OpTerm(Expr::Operation(
                                expr.into_iter().nth(0).unwrap(),
                            ))],
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
