use crate::ast::{BlockContents, Expr, Op, Stmt};
use std::collections::HashMap;

pub struct Evaluator {
    env: HashMap<String, i64>,
    pub output: Vec<i64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            output: Vec::new(),
        }
    }

    pub fn eval_program(&mut self, program: Vec<Stmt>) -> Vec<i64> {
        for stmt in program {
            self.eval_statement(stmt);
        }
        self.output.clone()
    }

    fn eval_statement(&mut self, stmt: Stmt) -> i64 {
        match stmt {
            Stmt::Let { name, value } => {
                let result = self.eval_expression(value);
                self.env.insert(name, result);
                result
            }
            Stmt::Print(expr) => {
                let result = self.eval_expression(expr);
                println!("{}", result);
                self.output.push(result);
                result
            }
            Stmt::Expression(expr) => self.eval_expression(expr),
            Stmt::While { condition, body } => {
                while self.eval_expression((*condition).clone()) != 0 {
                    self.eval_block_contents((*body).clone());
                }
                0
            }
            Stmt::Function { name, params, body } => todo!(),
            Stmt::Return(expr) => todo!(),
        }
    }

    fn eval_block_contents(&mut self, block: BlockContents) -> i64 {
        let mut result = 0;
        for stmt in block.statements {
            result = self.eval_statement(stmt);
        }
        result
    }

    fn eval_expression(&mut self, expr: Expr) -> i64 {
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
                    Op::Eq => {
                        if left_val == right_val {
                            1
                        } else {
                            0
                        }
                    }
                    Op::NotEq => {
                        if left_val != right_val {
                            1
                        } else {
                            0
                        }
                    }
                    Op::Lt => {
                        if left_val < right_val {
                            1
                        } else {
                            0
                        }
                    }
                    Op::Gt => {
                        if left_val > right_val {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
            Expr::Bool(bool) => {
                if bool {
                    1
                } else {
                    0
                }
            }
            Expr::If {
                condition,
                consequence,
                alternative,
            } => {
                let condition_val = self.eval_expression(*condition);
                if condition_val != 0 {
                    self.eval_block_contents(*consequence)
                } else {
                    if let Some(alt) = alternative {
                        self.eval_block_contents(*alt)
                    } else {
                        0
                    }
                }
            }
            Expr::FunctionCall { .. } => todo!(),
        }
    }
}
