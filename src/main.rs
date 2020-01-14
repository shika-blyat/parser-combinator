mod combinators;
mod common;
mod error;
mod math;
mod parser;

use math::take_number;
use parser::build_ast;

fn main() {
    let s = "(1 + 3) * 5".to_string();
    println!("{:#?}", build_ast()(s));
}
