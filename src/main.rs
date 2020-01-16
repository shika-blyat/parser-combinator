mod combinators;
mod common;
mod error;
mod evaluation;
mod math;
mod parser;
mod typechecking;

use error::ParserError;
use evaluation::eval_ast;
use parser::{build_ast, Literal};
use typechecking::type_ast;

fn eval_input<'a>(s: &'a str) -> Result<Literal, ParserError> {
    let ast = type_ast()(build_ast()(s.to_string()).unwrap().1)?;
    eval_ast(ast.1)
}

fn main() {
    println!("{:#?}", build_ast()("(1 + 2) * 3 / 4".to_string()));
}
