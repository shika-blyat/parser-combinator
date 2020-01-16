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
    eval_ast(type_ast()(build_ast()(s.to_string()).unwrap().1)?.1)
}
fn main() {
    println!("{:#?}", eval_input("1 + 2"));
}
