use crate::ast::Value;
use crate::ast::AST;
use crate::tokenizer::Tokenizer;

enum ParserError {
    EOF,
    UnexpectedToken(String),
    UnexpectedEOF,
}

struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    fn new(path: &str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(path),
        }
    }

    /*
    true = lambda x. lambda y. x;
    ===
    (define _true (lambda (x) (lambda (y) x)))

    Value::Eval(
        "define",
        Value::AnonFunc(
            "x",
            Value::AnonFunc(
                "y",
                Value::Parameter("x")
            )
        )
    )

    */

    fn parse_block(&mut self) -> Result<Value, ParserError> {
        Err(ParserError::EOF)
    }

    fn parse(&mut self) -> AST {
        let mut ast = AST::new();
        loop {
            let val = self.parse_block();
            match val {
                Ok(val) => {
                    ast.add(val);
                }
                Err(err) => match err {
                    ParserError::EOF => return ast,
                    _ => panic!("panik"),
                },
            }
        }
    }
}
