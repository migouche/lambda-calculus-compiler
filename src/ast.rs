use std::rc::Rc;

pub(crate) enum Value {
    Eval(String, Rc<Value>),     // name, arg
    AnonFunc(String, Rc<Value>), // arg, val
    Arg(String),
}

pub(crate) struct AST {
    pub program: Vec<Value>,
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

impl AST {
    pub fn new() -> AST {
        AST {
            program: Vec::new(),
        }
    }

    pub fn add(&mut self, Value: Value) {
        self.program.push(Value);
    }
}
