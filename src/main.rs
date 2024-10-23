use std::collections::VecDeque;

use keepsplit::KeepSplit;
use tokenizer::Tokenizer;

mod ast;
mod keepsplit;
mod parser;
mod tokenizer;

fn main() {
    let split_result = ".blabla.bal.".keepsplit('.');

    println!("{:?}", split_result);

    println!("Hello, world!");
    let v = VecDeque::from([1, 2, 3, 4]);
    println!("{:?}", v.as_slices());

    let mut t = Tokenizer::new("test.txt");
    while let Some(token) = t.next_token() {
        println!("{}", &token);
    }
}
