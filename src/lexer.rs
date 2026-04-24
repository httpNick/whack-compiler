use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position];

        let token = match ch {
            '=' => {
                self.position += 1;
                if self.position < self.input.len() && self.input[self.position] == '=' {
                    self.position += 1;
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                self.position += 1;
                if self.position < self.input.len() && self.input[self.position] == '=' {
                    self.position += 1;
                    Token::NotEq
                } else {
                    Token::Illegal(ch)
                }
            }
            '<' => {
                self.position += 1;
                Token::Lt
            }
            '>' => {
                self.position += 1;
                Token::Gt
            }
            '{' => {
                self.position += 1;
                Token::LBrace
            }
            '}' => {
                self.position += 1;
                Token::RBrace
            }
            ';' => {
                self.position += 1;
                Token::Semicolon
            }
            '(' => {
                self.position += 1;
                Token::LParen
            }
            ')' => {
                self.position += 1;
                Token::RParen
            }
            '+' => {
                self.position += 1;
                Token::Plus
            }
            '-' => {
                self.position += 1;
                Token::Minus
            }
            '*' => {
                self.position += 1;
                Token::Star
            }
            '/' => {
                self.position += 1;
                Token::Slash
            }
            '%' => {
                self.position += 1;
                Token::Modulo
            }
            ',' => {
                self.position += 1;
                Token::Comma
            }
            _ => {
                if ch.is_alphabetic() {
                    return self.read_identifier();
                } else if ch.is_numeric() {
                    return self.read_number();
                } else {
                    self.position += 1;
                    Token::Illegal(ch)
                }
            }
        };

        token
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while self.position < self.input.len()
            && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_')
        {
            identifier.push(self.input[self.position]);
            self.position += 1;
        }

        match identifier.as_str() {
            "let" => Token::Let,
            "print" => Token::Print,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "true" => Token::True,
            "false" => Token::False,
            "fn" => Token::Fn,
            "return" => Token::Return,
            _ => Token::Ident(identifier),
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number_str = String::new();
        while self.position < self.input.len() && self.input[self.position].is_numeric() {
            number_str.push(self.input[self.position]);
            self.position += 1;
        }
        Token::Integer(number_str.parse().unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let source = "let x = 5;\nprint x + 10;\nif (x == 10) {\n    print x;\n}";
        let mut lexer = Lexer::new(source.to_string());

        let expected_tokens = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Print,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Integer(10),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Eq,
            Token::Integer(10),
            Token::RParen,
            Token::LBrace,
            Token::Print,
            Token::Ident("x".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::EOF,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_next_token_illegal_chars() {
        let source = "let x =! 5;";
        let mut lexer = Lexer::new(source.to_string());

        let expected_tokens = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Illegal('!'),
            Token::Integer(5),
            Token::Semicolon,
            Token::EOF,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }
}
