mod ast;
mod compiler;
mod eval;
mod lexer;
mod parser;
mod token;

use compiler::Compiler;
use eval::Evaluator;
use lexer::Lexer;
use parser::Parser;
use std::fs;
use std::io::Write;

fn main() {
    let source =
        fs::read_to_string("test.whack").expect("Could not read test.whack. Make sure it exists!");

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("--- Interpreter Output ---");
    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program.clone());

    println!("\n--- Compiling to C ---");
    let mut compiler = Compiler::new();
    let c_code = compiler.compile(program);

    let mut file = fs::File::create("output.c").expect("Could not create output.c");
    file.write_all(c_code.as_bytes())
        .expect("Could not write to output.c");

    println!("Success! Created 'output.c'.");
    println!("Now run: gcc output.c -o whack_app && ./whack_app");
}
