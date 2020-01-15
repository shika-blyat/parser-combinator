mod combinators;
mod common;
mod error;
mod math;
mod parser;
mod typechecking;

use parser::build_ast;
use typechecking::type_ast;

fn main() {
    let s = "(1.0 + 3.0) * 3.0 * 5.0".to_string();
    println!("{:#?}", type_ast()(build_ast()(s).unwrap().1));
}
