mod combinators;
mod common;
mod error;
mod evaluation;
mod math;
mod parser;
mod typechecking;

use error::ParserError;
use evaluation::eval_ast;
use parser::{build_ast, take_var, Expr, Literal};
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};
use typechecking::type_ast;

struct Environment {
    variables: HashMap<String, Expr>,
}
impl Environment {
    pub fn new() -> Self {
        let variables = HashMap::new();
        Self { variables }
    }
}
fn eval_input<'a>(s: &'a str, variables: &'a mut HashMap<String, Expr>) {
    let ast = take_var()((s.to_string(), variables)).map_err(|error| {
        build_ast()(error.remaining()).map(|ast| println!("{:#?}", eval_ast(ast.1)))
    });
    println!("{:#?}", variables);
}
fn main() {
    let mut variables = HashMap::new();
    loop {
        let mut input = String::new();
        print!(">>> ");
        stdout().flush().expect("Failed to write line");
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "quit" {
            break;
        }
        let r = eval_input(input, &mut variables);
    }
}
