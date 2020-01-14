use crate::combinators::{many, many1};
use crate::common::{take_char, take_digit, take_str};
use crate::error::ParserError;
use crate::parser::{Assoc, Atom, Expr, Number, Operator, Parser};
pub enum Bin {
    Binary {
        left: Box<Bin>,
        op: Operator,
        right: Box<Bin>,
    },
    Atom(Atom),
}
impl Bin {
    pub fn new_bin(lbin: Bin, op: Operator, rbin: Bin) -> Self {
        let (left, right) = (Box::new(lbin), Box::new(rbin));
        Self::Binary { left, op, right }
    }
    pub fn new_atom(atom: Atom) -> Self {
        Self::Atom(atom)
    }
}

pub fn into_postfix() -> Parser<Expr, Expr> {
    Box::new(|tokens| {
        let mut op_stack: Expr = vec![];
        let mut output = vec![];
        let ast: Option<Bin> = None;
        for i in tokens {
            match i {
                Atom::Lit(lit) => output.push(Atom::Lit(lit)),
                Atom::Var(ident) => output.push(Atom::Var(ident)),
                Atom::Op(op) => {
                    while op_stack.last().is_some() {
                        let last = op_stack.last().unwrap();
                        if let Atom::Op(last_op) = last {
                            if last_op.precedence >= op.precedence {
                                output.push(op_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    op_stack.push(Atom::Op(op));
                }
                Atom::Parens(expr) => into_postfix()(expr)?
                    .1
                    .into_iter()
                    .for_each(|x| output.push(x)),
            }
        }
        for i in op_stack.into_iter().rev() {
            output.push(i);
        }
        Ok(("".to_string(), output))
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

pub fn take_operator() -> Parser<Atom, String> {
    Box::new(|s| {
        take_str("+".to_string())(s.clone())
            .or_else(|error| take_str("*".to_string())(error.remaining()))
            .or_else(|error| take_str("/".to_string())(error.remaining()))
            .or_else(|error| take_str("-".to_string())(error.remaining()))
            .and_then(|(remaining, op)| match op.as_str() {
                "+" => Ok((
                    remaining,
                    Atom::Op(Operator {
                        lexeme: op,
                        precedence: 5,
                        assoc: Assoc::Left,
                    }),
                )),
                "*" => Ok((
                    remaining,
                    Atom::Op(Operator {
                        lexeme: op,
                        precedence: 10,
                        assoc: Assoc::Left,
                    }),
                )),
                "/" => Ok((
                    remaining,
                    Atom::Op(Operator {
                        lexeme: op,
                        precedence: 10,
                        assoc: Assoc::Left,
                    }),
                )),
                "-" => Ok((
                    remaining,
                    Atom::Op(Operator {
                        lexeme: op,
                        precedence: 5,
                        assoc: Assoc::Left,
                    }),
                )),
                _ => Err(ParserError::new(s, format!("Unknwon operator: {}", op))),
            })
    })
}
