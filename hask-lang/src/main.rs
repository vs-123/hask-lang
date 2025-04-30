use std::fs::{self, File};
use std::env::args;
mod parser;
mod compiler;
pub mod ast;

fn main() {
    let mut args = args();

    args.next().unwrap();
    
    let input_file = args.next().unwrap();

    let code = fs::read_to_string(input_file.clone()).unwrap();
    let ast = parser::ProgramParser::new().parse(&code).unwrap();

    let out = compiler::compile(&ast);

    fs::write(format!("{}.c", input_file.clone()), out.as_bytes());
    println!("Successfully compiled {}!", input_file.clone());
}