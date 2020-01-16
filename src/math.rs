use crate::combinators::{many, many1};
use crate::common::{take_char, take_digit, take_str};
use crate::error::ParserError;
use crate::parser::{Assoc, Bin, Expr, Number, OpTerm, Operator, Parser};

fn add_infix_op(ast: &mut Vec<Bin>, operator: Operator) {
    let roperand = ast.pop().unwrap();
    let loperand = ast.pop().unwrap();
    ast.push(Bin::new_bin(
        Expr::BinOp(Box::new(loperand)),
        operator,
        Expr::BinOp(Box::new(roperand)),
    ));
}
pub fn into_ast() -> Parser<Bin, Vec<OpTerm>> {
    Box::new(|tokens| {
        let mut op_stack: Vec<OpTerm> = vec![];
        let mut ast: Vec<Bin> = vec![];
        for i in tokens.into_iter() {
            match i {
                OpTerm::OpTerm(Expr::Lit(lit)) => ast.push(Bin::new_uno(Expr::Lit(lit))),
                OpTerm::OpTerm(Expr::Var(ident)) => ast.push(Bin::new_uno(Expr::Var(ident))),
                OpTerm::Op(op) => {
                    while op_stack.last().is_some() {
                        let last = op_stack.last().unwrap();
                        if let OpTerm::Op(last_op) = last {
                            if last_op.precedence >= op.precedence {
                                let operator = match op_stack.pop().unwrap() {
                                    OpTerm::Op(op) => op,
                                    _ => unreachable!(),
                                };
                                add_infix_op(&mut ast, operator);
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    op_stack.push(OpTerm::Op(op));
                }
                OpTerm::OpTerm(Expr::Operation(expr)) => ast.push(into_ast()(expr)?.1),
                _ => unreachable!(),
            }
        }
        for i in op_stack.into_iter().rev() {
            if let OpTerm::Op(op) = i {
                add_infix_op(&mut ast, op);
            } else {
                unreachable!();
            }
        }
        Ok(("".to_string(), ast.into_iter().nth(0).unwrap()))
    })
}

pub fn take_number() -> Parser<Number, String> {
    Box::new(|s| {
        let (remaining, mut num) = many1(take_digit())(s)?;
        match take_char('.')(remaining) {
            Ok((remaining, dot)) => {
                num.push(dot);
                let (remaining, mut decimals) = many(take_digit())(remaining)?;
                num.append(&mut decimals);
                Ok((
                    remaining,
                    Number::F32(num.iter().collect::<String>().parse().unwrap()),
                ))
            }
            Err(error) => Ok((
                error.remaining(),
                Number::I32(num.iter().collect::<String>().parse().unwrap()),
            )),
        }
    })
}

pub fn take_operator() -> Parser<OpTerm, String> {
    Box::new(|s| {
        take_str("+".to_string())(s.clone())
            .or_else(|error| take_str("*".to_string())(error.remaining()))
            .or_else(|error| take_str("/".to_string())(error.remaining()))
            .or_else(|error| take_str("-".to_string())(error.remaining()))
            .and_then(|(remaining, op)| match op.as_str() {
                "+" => Ok((
                    remaining,
                    OpTerm::Op(Operator {
                        lexeme: op,
                        precedence: 5,
                        assoc: Assoc::Left,
                    }),
                )),
                "*" => Ok((
                    remaining,
                    OpTerm::Op(Operator {
                        lexeme: op,
                        precedence: 10,
                        assoc: Assoc::Left,
                    }),
                )),
                "/" => Ok((
                    remaining,
                    OpTerm::Op(Operator {
                        lexeme: op,
                        precedence: 10,
                        assoc: Assoc::Left,
                    }),
                )),
                "-" => Ok((
                    remaining,
                    OpTerm::Op(Operator {
                        lexeme: op,
                        precedence: 5,
                        assoc: Assoc::Left,
                    }),
                )),
                _ => Err(ParserError::new(s, format!("Unknwon operator: {}", op))),
            })
    })
}

#[test]
fn ast() {
    use crate::parser::{build_ast, Binary, Literal};

    assert_eq!(
        build_ast()("(1 + 2) * 3".to_string()),
        Ok((
            "".to_string(),
            Bin::Bin(Binary {
                left: Expr::BinOp(Box::new(Bin::Bin(Binary {
                    left: Expr::BinOp(Box::new(Bin::Uno(Expr::Lit(Literal::Num(Number::I32(1)))))),
                    op: Operator {
                        lexeme: "+".to_string(),
                        precedence: 5,
                        assoc: Assoc::Left,
                    },
                    right: Expr::BinOp(Box::new(Bin::Uno(Expr::Lit(Literal::Num(Number::I32(2)))))),
                    expr_type: None,
                }))),
                op: Operator {
                    lexeme: "*".to_string(),
                    precedence: 10,
                    assoc: Assoc::Left,
                },
                right: Expr::BinOp(Box::new(Bin::Uno(Expr::Lit(Literal::Num(Number::I32(3)))))),
                expr_type: None,
            }),
        ))
    );
    assert_eq!(
        build_ast()("1 + 2 * 3".to_string()),
        Ok((
            "".to_string(),
            Bin::Bin(Binary {
                left: Expr::BinOp(Box::new(Bin::Uno(Expr::Lit(Literal::Num(Number::I32(1)))))),
                op: Operator {
                    lexeme: "+".to_string(),
                    precedence: 5,
                    assoc: Assoc::Left,
                },
                right: Expr::BinOp(Box::new(Bin::Bin(Binary {
                    left: Expr::BinOp(Box::new(Bin::Uno(Expr::Lit(Literal::Num(Number::I32(2)))))),
                    op: Operator {
                        lexeme: "*".to_string(),
                        precedence: 10,
                        assoc: Assoc::Left,
                    },
                    right: Expr::BinOp(Box::new(Bin::Uno(Expr::Lit(Literal::Num(Number::I32(3)))))),
                    expr_type: None,
                }))),
                expr_type: None,
            }),
        ))
    );
}
