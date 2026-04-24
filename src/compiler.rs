use std::collections::HashSet;

use crate::ast::{BlockContents, Expr, Op, Stmt};

pub struct Compiler {
    output: String,
    vars: HashSet<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            output: String::from("#include <stdio.h>\n\nint main() {\n"),
            vars: HashSet::new(),
        }
    }

    pub fn compile(&mut self, program: Vec<Stmt>) -> String {
        for stmt in program {
            let (code, _) = self.compile_statement(stmt);
            self.output.push_str(&code);
        }
        self.output.push_str("    return 0;\n}\n");
        self.output.clone()
    }

    fn compile_statement(&mut self, stmt: Stmt) -> (String, String) {
        match stmt {
            Stmt::Let { name, value } => {
                let expr_str = self.compile_expression(value);
                (
                    if self.vars.contains(&name) {
                        format!("    {} = {};\n", name, expr_str)
                    } else {
                        self.vars.insert(name.clone());
                        format!("    long {} = {};\n", name, expr_str)
                    },
                    name.to_string(),
                )
            }
            Stmt::Print(expr) => {
                let expr_str = self.compile_expression(expr);
                (
                    format!("    printf(\"%ld\\n\", {});\n", expr_str),
                    String::new(),
                )
            }
            Stmt::Expression(expr) => {
                let expr_str = self.compile_expression(expr);
                (format!("    {};\n", expr_str), expr_str)
            }
            Stmt::While { condition, body } => {
                let condition_str = self.compile_expression(*condition);
                let body_str = self.compile_block_contents(*body, "");
                (
                    format!("    while ({}) {{\n{}\n    }}\n", condition_str, body_str),
                    String::new(),
                )
            }
            Stmt::Function { name, params, body } => todo!(),
            Stmt::Return(expr) => todo!(),
        }
    }

    fn compile_block_contents(&mut self, block: BlockContents, result_var: &str) -> String {
        let mut result = String::new();
        for (i, stmt) in block.statements.iter().enumerate() {
            let is_last = i == block.statements.len() - 1;
            let (code, var) = self.compile_statement(stmt.clone());
            result.push_str(&code);
            if is_last && !var.is_empty() && !result_var.is_empty() {
                result.push_str(&format!("    {} = {};\n", result_var, var));
            }
        }
        result
    }

    fn compile_expression(&mut self, expr: Expr) -> String {
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
                    Op::Eq => "==",
                    Op::NotEq => "!=",
                    Op::Lt => "<",
                    Op::Gt => ">",
                };
                format!("({} {} {})", left_str, op_str, right_str)
            }
            Expr::If {
                condition,
                consequence,
                alternative,
            } => {
                let condition_str = self.compile_expression(*condition);
                let consequence_str = self.compile_block_contents(*consequence, "_res");
                let alternative_str = if let Some(alt) = alternative {
                    self.compile_block_contents(*alt, "_res")
                } else {
                    String::new()
                };
                format!(
                    "({{
long _res = 0;
if ({}) {{
{}
}} else {{
{}
}}
_res;
}})",
                    condition_str, consequence_str, alternative_str
                )
            }
            Expr::Bool(b) => if b { "1" } else { "0" }.to_string(),
            Expr::FunctionCall { .. } => todo!(),
        }
    }
}
