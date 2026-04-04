use crate::ast::{Expr, Op, Stmt};
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(PartialOrd, PartialEq, Clone, Copy, Debug)]
enum Precedence {
    Lowest,
    Sum,     // + or -
    Product, // * or /
    Prefix,  // -5
    Call,    // func()
}

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while self.cur_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        statements
    }

    fn parse_statement(&mut self) -> Option<Stmt> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Print => self.parse_print_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Stmt> {
        self.next_token();

        let name = if let Token::Ident(ref name) = self.cur_token {
            name.clone()
        } else {
            return None;
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        self.next_token();

        let value = if let Some(exp) = self.parse_expression(Precedence::Lowest) {
            exp
        } else {
            return None;
        };

        if !self.expect_peek(Token::Semicolon) {
            return None;
        }

        Some(Stmt::Let { name, value })
    }

    fn parse_print_statement(&mut self) -> Option<Stmt> {
        self.next_token();
        let value = if let Some(exp) = self.parse_expression(Precedence::Lowest) {
            exp
        } else {
            return None;
        };

        if !self.expect_peek(Token::Semicolon) {
            return None;
        }

        Some(Stmt::Print(value))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left_expr = match self.cur_token {
            Token::Integer(val) => Some(Expr::Literal(val)),
            Token::Ident(ref name) => Some(Expr::Variable(name.clone())),
            Token::LParen => self.parse_grouped_expression(),
            _ => None,
        };

        while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
            self.next_token();
            left_expr = self.parse_infix_expression(left_expr.unwrap());
        }

        left_expr
    }

    fn parse_infix_expression(&mut self, left: Expr) -> Option<Expr> {
        let op = match self.cur_token {
            Token::Plus => Op::Plus,
            Token::Minus => Op::Minus,
            Token::Star => Op::Multiply,
            Token::Slash => Op::Divide,
            Token::Modulo => Op::Modulo,
            _ => return None,
        };

        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Some(Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expr> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        Some(expr)
    }

    fn peek_precedence(&self) -> Precedence {
        match self.peek_token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Star | Token::Slash | Token::Modulo => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn cur_precedence(&self) -> Precedence {
        match self.cur_token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Star | Token::Slash | Token::Modulo => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token == token {
            self.next_token();
            true
        } else {
            eprintln!("Expected token {:?}, got {:?}", token, self.peek_token);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_let_statement() {
        let source = "let x = 5;";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        if let Stmt::Let { name, value } = &program[0] {
            assert_eq!(name, "x");
            assert_eq!(value, &Expr::Literal(5));
        } else {
            panic!("Expected Stmt::Let, got {:?}", &program[0]);
        }
    }

    #[test]
    fn test_parse_let_statement_with_multiply() {
        let source = "let x = 5 + 10 * 2;";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        if let Stmt::Let { name, value } = &program[0] {
            assert_eq!(name, "x");
            assert_eq!(
                value,
                &Expr::Binary {
                    left: Box::new(Expr::Literal(5)),
                    op: Op::Plus,
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::Literal(10)),
                        op: Op::Multiply,
                        right: Box::new(Expr::Literal(2)),
                    }),
                }
            );
        } else {
            panic!("Expected Stmt::Let, got {:?}", &program[0]);
        }
    }

    #[test]
    fn test_parse_let_statement_with_parens() {
        let source = "let x = (5 + 10) * 2;";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        if let Stmt::Let { name, value } = &program[0] {
            assert_eq!(name, "x");
            assert_eq!(
                value,
                &Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Literal(5)),
                        op: Op::Plus,
                        right: Box::new(Expr::Literal(10)),
                    }),
                    op: Op::Multiply,
                    right: Box::new(Expr::Literal(2)),
                }
            );
        } else {
            panic!("Expected Stmt::Let, got {:?}", &program[0]);
        }
    }
}
