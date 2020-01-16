use crate::error::ParserError;
use crate::parser::{Bin, Binary, Expr, Operator, Parser, Type};

impl Bin {
    pub fn into_typed(self) -> Result<Bin, ParserError> {
        match self {
            Bin::Bin(Binary {
                mut left,
                mut right,
                op,
                ..
            }) => {
                let left = match left.unwrap_bin() {
                    Some(bin) => Box::new(bin.clone().into_typed()?),
                    None => unreachable!(),
                };
                let right = match right.unwrap_bin() {
                    Some(bin) => Box::new(bin.clone().into_typed()?),
                    None => unreachable!(),
                };
                let expr_type = binary_type(left.get_type(), &op, right.get_type())?;
                Ok(Bin::new_bin_typed(
                    Expr::BinOp(left),
                    op,
                    Expr::BinOp(right),
                    expr_type,
                ))
            }
            Bin::Uno(expr) => Ok(Bin::Uno(expr)),
        }
    }
}
fn binary_type(left: Type, op: &Operator, right: Type) -> Result<Type, ParserError> {
    match left {
        Type::U32 => match right {
            Type::U32 => match op.lexeme.as_str() {
                "+" => Ok(Type::U32),
                "*" => Ok(Type::U32),
                "-" => Ok(Type::U32),
                "/" => Ok(Type::F32),
                _ => Err(ParserError::new_no_rem(format!(
                    "Unknown operator `{}`",
                    op.lexeme
                ))),
            },
            Type::I32 => Err(ParserError::new_no_rem(
                "Cannot add an u32 and an i32 between them".to_string(),
            )),
            Type::F32 => Err(ParserError::new_no_rem(
                "Cannot add an u32 and an f32 between them".to_string(),
            )),
        },
        Type::I32 => match right {
            Type::I32 => match op.lexeme.as_str() {
                "+" => Ok(Type::I32),
                "*" => Ok(Type::I32),
                "-" => Ok(Type::I32),
                "/" => Ok(Type::F32),
                _ => Err(ParserError::new_no_rem(format!(
                    "Unknown operator `{}`",
                    op.lexeme
                ))),
            },
            Type::U32 => Err(ParserError::new_no_rem(
                "Cannot add an i32 and an u32 between them".to_string(),
            )),
            Type::F32 => Err(ParserError::new_no_rem(
                "Cannot add an i32 and an f32 between them".to_string(),
            )),
        },
        Type::F32 => match right {
            Type::F32 => match op.lexeme.as_str() {
                "+" => Ok(Type::F32),
                "*" => Ok(Type::F32),
                "-" => Ok(Type::F32),
                "/" => Ok(Type::F32),
                _ => Err(ParserError::new_no_rem(format!(
                    "Unknown operator `{}`",
                    op.lexeme
                ))),
            },
            Type::U32 => Err(ParserError::new_no_rem(
                "Cannot add an f32 and an u32 between them".to_string(),
            )),
            Type::I32 => Err(ParserError::new_no_rem(
                "Cannot add an f32 and an i32 between them".to_string(),
            )),
        },
    }
}

pub fn type_ast() -> Parser<Bin, Bin> {
    Box::new(|bin| {
        let typed_bin = bin.into_typed()?;
        Ok(("".to_string(), typed_bin))
    })
}
