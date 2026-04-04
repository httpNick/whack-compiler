use crate::ast::{Expr, Op, Stmt};

pub struct Compiler {
    output: String,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            output: String::from("#include <stdio.h>\n\nint main() {\n"),
        }
    }

    pub fn compile(&mut self, program: Vec<Stmt>) -> String {
        for stmt in program {
            self.compile_statement(stmt);
        }
        self.output.push_str("    return 0;\n}\n");
        self.output.clone()
    }

    fn compile_statement(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                let expr_str = self.compile_expression(value);
                self.output
                    .push_str(&format!("    long {} = {};\n", name, expr_str));
            }
            Stmt::Print(expr) => {
                let expr_str = self.compile_expression(expr);
                self.output
                    .push_str(&format!("    printf(\"%ld\\n\", {});\n", expr_str));
            }
            Stmt::Expression(expr) => {
                let expr_str = self.compile_expression(expr);
                self.output.push_str(&format!("    {};\n", expr_str));
            }
        }
    }

    fn compile_expression(&self, expr: Expr) -> String {
        match expr {
            Expr::Literal(val) => val.to_string(),
            Expr::Variable(name) => name,
            Expr::Binary { left, op, right } => {
                let left_str = self.compile_expression(*left);
                let right_str = self.compile_expression(*right);
                let op_str = match op {
                    Op::Plus => "+",
                    Op::Minus => "-",
                    Op::Multiply => "*",
                    Op::Divide => "/",
                    Op::Modulo => "%",
                };
                format!("({} {} {})", left_str, op_str, right_str)
            }
        }
    }
}
