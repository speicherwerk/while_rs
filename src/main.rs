use std::env::args;
use std::fs::read_to_string;

use while_rs::eval::Context;
use while_rs::while_grammar::StatementParser;

fn main() {
    let file_path = match args().nth(1) {
        Some(s) => s,
        None => panic!("Expected file path!"),
    };
    match StatementParser::new().parse(&read_to_string(file_path).expect("Unable to read file")) {
        Ok(ast) => {
            println!("AST:");
            println!("{ast:#?}");
            println!("\n--- Begin of program output\n");
            let mut ctx = Context::default();
            ast.eval(&mut ctx);
        }
        Err(e) => println!("Error: {e}"),
    }
}
