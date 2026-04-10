use crate::ast::{BlockContents, Expr, Op, Stmt};
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(PartialOrd, PartialEq, Clone, Copy, Debug)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
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
            _ => {
                if let Some(expr) = self.parse_expression(Precedence::Lowest) {
                    Some(Stmt::Expression(expr))
                } else {
                    None
                }
            }
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
            Token::True => Some(Expr::Bool(true)),
            Token::False => Some(Expr::Bool(false)),
            Token::If => self.parse_if_expression(),
            _ => None,
        };

        while self.peek_token != Token::Semicolon
            && self.peek_token != Token::RParen
            && self.peek_token != Token::RBrace
            && precedence < self.peek_precedence()
        {
            self.next_token();
            left_expr = self.parse_infix_expression(left_expr?);
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
            Token::Eq => Op::Eq,
            Token::NotEq => Op::NotEq,
            Token::Lt => Op::Lt,
            Token::Gt => Op::Gt,
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

    fn parse_if_expression(&mut self) -> Option<Expr> {
        if !self.expect_peek(Token::LParen) {
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        if !self.expect_peek(Token::LBrace) {
            return None;
        }
        let consequence = self.parse_block_contents()?;
        let alternative = if self.peek_token == Token::Else {
            self.next_token();
            if !self.expect_peek(Token::LBrace) {
                return None;
            }
            Some(self.parse_block_contents()?)
        } else {
            None
        };
        Some(Expr::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: alternative.map(Box::new),
        })
    }

    fn parse_block_contents(&mut self) -> Option<BlockContents> {
        let mut statements = Vec::new();
        self.next_token();
        while self.cur_token != Token::RBrace && self.cur_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        Some(BlockContents { statements })
    }

    fn peek_precedence(&self) -> Precedence {
        match self.peek_token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Star | Token::Slash | Token::Modulo => Precedence::Product,
            Token::Eq | Token::NotEq => Precedence::Equals,
            Token::Lt | Token::Gt => Precedence::LessGreater,
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
