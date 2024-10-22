use core::str;
use std::rc::Rc;

enum Value {
    Eval(Rc<Variable>, Rc<Value>),
    AnonFunc(LamdaFunction),
    Arg(String)
}

struct Variable{
    name: String,
    val: Value
}

struct LamdaFunction{
    argname: String,
    value: Rc<Value>
}

struct AST{
    pub program: Vec<Variable>
}

/*
true = lambda x. lambda y. x;

Variable(
    "true",
    Value::AnonFunc(
        "x",
        Value::AnonFunc(
            "y",
            Value::Arg("x")
        ))
    )

*/