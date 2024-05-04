use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::slice::Iter;
use crate::lexer::Token;

struct Parser<'a> {
    iter: &'a mut Peekable<Iter<'a, Token>>
}

impl<'a> Parser<'a> {
    fn new(iter: &'a mut Peekable<Iter<'a, Token>>) -> Self {
        Self { iter }
    }

    fn assert_next(&mut self, token: Token) -> Result<(), SyntaxError> {
        let next = self.iter.next();

        if let None = next {
            return Err(SyntaxError::new_parse_error("Unexpected end of input".to_string()))
        }

        if *next.unwrap() != token {
            return Err(SyntaxError::new_parse_error(format!("Expected {:?} token, got {:?}", token, next.unwrap())));
        }

        Ok(())
    }

}

#[derive(Debug)]
enum Expression {
    StringValue(String),
    BoolValue(bool),
    IntValue(i64),
    FloatValue(f64),
    FCall(String, Option<Vec<Box<Expression>>>),
    Constructor(String, Option<Vec<Box<Expression>>>),
    GenCall(String, Option<Vec<String>>)
}

impl Expression {
    fn eval(&mut self) -> String {
        match self {
            Expression::StringValue(v) => v.clone(),
            Expression::BoolValue(v) => v.to_string(),
            Expression::IntValue(v) => v.to_string(),
            Expression::FloatValue(v) => v.to_string(),
            Expression::FCall(fun, args) => {}
            Expression::Constructor(class, args) => {}
            Expression::GenCall(generator, args) => {}
        }
    }
}

enum Declaration {
    Data(String, Vec<String>, Vec<(String, Box<Expression>)>),
    Const(String, Box<Expression>),
    Var(String, Box<Expression>),
    Gen(String, Vec<String>, Box<Expression>),
    Main(Box<Expression>)
}


#[derive(Debug)]
struct SyntaxError {
    message: String,
    level: String,
}

impl SyntaxError {
    fn new_parse_error(message: String) -> Self {
        SyntaxError {
            message,
            level: "PARSER".to_string(),
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ERROR {}", self.level, self.message)
    }
}

impl Error for SyntaxError {}