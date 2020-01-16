use crate::error::ParserError;
use crate::parser::{Bin, Binary, Expr, Literal, Operator};

pub fn eval_ast(bin: Bin) -> Result<Literal, ParserError> {
    match bin {
        Bin::Bin(Binary {
            left, right, op, ..
        }) => {
            let left = match left {
                Expr::Lit(lit) => lit,
                Expr::BinOp(_) => eval_ast(left.to_bin())?,
                _ => {
                    return Err(ParserError::new_no_rem(format!(
                        "evaluation failed because {:#?} was inside the tree",
                        left
                    )))
                }
            };
            let right = match right {
                Expr::Lit(lit) => lit,
                Expr::BinOp(_) => eval_ast(right.to_bin())?,
                _ => {
                    return Err(ParserError::new_no_rem(format!(
                        "evaluation failed because {:#?} was inside the tree",
                        right
                    )))
                }
            };
            eval_bin(left, op, right)
        }
        Bin::Uno(expr) => match expr {
            Expr::Lit(lit) => Ok(lit),
            _ => panic!(),
        },
    }
}

pub fn eval_bin(left: Literal, op: Operator, right: Literal) -> Result<Literal, ParserError> {
    match left {
        Literal::Num(lnum) => match right {
            Literal::Num(rnum) => match op.lexeme.as_str() {
                "+" => Ok(Literal::Num(lnum + rnum)),
                "-" => Ok(Literal::Num(lnum - rnum)),
                "*" => Ok(Literal::Num(lnum * rnum)),
                "/" => Ok(Literal::Num(lnum / rnum)),
                _ => panic!(),
            },
        },
    }
}

#[test]
fn eval() {
    use crate::eval_input;
    use crate::parser::Number;
    assert_eq!(eval_input("1+2"), Ok((Literal::Num(Number::I32(3)))));
    assert_eq!(
        eval_input("(1+2) * 3 / 4"),
        Ok((Literal::Num(Number::I32((1 + 2) * 3 / 4))))
    );
}
