use std::collections::VecDeque;

use tokenizer::Tokenizer;

mod tokenizer;
mod ast;

fn main() {
    println!("Hello, world!");
    let v = VecDeque::from([1, 2, 3, 4]);
    println!("{:?}", v.as_slices());

    let mut t = Tokenizer::new("test.txt");
    while let Some(token) = t.next_token()
    {
        println!("{}", &token);
    }
}
