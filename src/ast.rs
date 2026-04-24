#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(i64),
    Bool(bool),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        consequence: Box<BlockContents>,
        alternative: Option<Box<BlockContents>>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Eq,
    NotEq,
    Lt,
    Gt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    Print(Expr),
    While {
        condition: Box<Expr>,
        body: Box<BlockContents>,
    },
    Expression(Expr),
    Function {
        name: String,
        params: Vec<String>,
        body: BlockContents,
    },
    Return(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockContents {
    pub statements: Vec<Stmt>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_creation() {
        let test_stmt = Stmt::Let {
            name: "x".to_string(),
            value: Expr::Literal(5),
        };

        if let Stmt::Let { name, value } = test_stmt {
            assert_eq!(name, "x");
            assert_eq!(value, Expr::Literal(5));
        } else {
            panic!("Expected Stmt::Let, got {:?}", test_stmt);
        }
    }

    #[test]
    fn test_binary_expression() {
        let test_expr = Expr::Binary {
            left: Box::new(Expr::Literal(5)),
            op: Op::Plus,
            right: Box::new(Expr::Literal(10)),
        };

        if let Expr::Binary { left, op, right } = test_expr {
            assert_eq!(left, Box::new(Expr::Literal(5)));
            assert_eq!(op, Op::Plus);
            assert_eq!(right, Box::new(Expr::Literal(10)));
        } else {
            panic!("Expected Expr::Binary, got {:?}", test_expr);
        }
    }
}
