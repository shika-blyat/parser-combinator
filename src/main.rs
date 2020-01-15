mod combinators;
mod common;
mod error;
mod math;
mod parser;
mod typechecking;

use math::take_number;
use parser::build_ast;
//use typechecking::type_ast;

fn main() {
    let s = "(1 + 3) * 3 * 5".to_string();
    //println!("{:#?}", type_ast()(build_ast()(s).unwrap().1));
}
