mod combinators;
mod common;
mod error;
mod math;
mod parser;

use math::{into_postfix, take_number};
use parser::{into_ast, take_expr};

fn main() {
    let s = "1 + 3 * 5".to_string();
    println!("{:?}", into_ast()(s));
    let s = "a".to_string();
    println!("{:?}", take_number()(s));
}
