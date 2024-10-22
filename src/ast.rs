use core::str;
use std::rc::Rc;

enum Value {
    Eval(Rc<LamdaFunction>, Rc<Value>),
    AnonFunc(String, Rc<LamdaFunction>)
}

struct LamdaFunction{
    argname: String,
    value: Rc<Value>
}
