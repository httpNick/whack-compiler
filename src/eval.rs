use crate::ast::{Expr, Op, Stmt};
use std::collections::HashMap;

pub struct Evaluator {
    env: HashMap<String, i64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn eval_program(&mut self, program: Vec<Stmt>) {
        for stmt in program {
            self.eval_statement(stmt);
        }
    }

    fn eval_statement(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                let result = self.eval_expression(value);
                self.env.insert(name, result);
            }
            Stmt::Print(expr) => {
                let result = self.eval_expression(expr);
                println!("{}", result);
            }
            Stmt::Expression(expr) => {
                self.eval_expression(expr);
            }
        }
    }

    fn eval_expression(&self, expr: Expr) -> i64 {
        match expr {
            Expr::Literal(val) => val,
            Expr::Variable(name) => *self.env.get(&name).unwrap_or_else(|| {
                eprintln!("Error: Undefined variable '{}'", name);
                &0
            }),
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expression(*left);
                let right_val = self.eval_expression(*right);

                match op {
                    Op::Plus => left_val + right_val,
                    Op::Minus => left_val - right_val,
                    Op::Multiply => left_val * right_val,
                    Op::Divide => {
                        if right_val == 0 {
                            eprintln!("Error: Division by zero");
                            0
                        } else {
                            left_val / right_val
                        }
                    }
                    Op::Modulo => {
                        if right_val == 0 {
                            eprintln!("Error: Division by zero");
                            0
                        } else {
                            left_val % right_val
                        }
                    }
                }
            }
        }
    }
}
