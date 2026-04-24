pub mod ast;
pub mod compiler;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod token;

pub use lexer::Lexer;
pub use parser::Parser;
pub use eval::Evaluator;
pub use compiler::Compiler;

pub fn interpret(source: &str) -> Result<Vec<i64>, String> {
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        return Err(parser.errors.join("\n"));
    }

    let mut evaluator = Evaluator::new();
    Ok(evaluator.eval_program(program))
}
