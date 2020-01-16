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
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};
use typechecking::type_ast;

fn eval_input<'a>(s: &'a str) -> Result<Literal, ParserError> {
    let ast = type_ast()(build_ast()(s.to_string()).unwrap().1)?;
    eval_ast(ast.1)
}
fn main() {
    //let mut variables = HashMap::new();
    loop {
        let mut input = String::new();
        print!(">>> ");
        stdout().flush().expect("Failed to write line");
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "quit" {
            break;
        }
        let r = eval_input(input);
        println!("{:#?}", r);
    }
}
