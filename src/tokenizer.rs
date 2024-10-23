use crate::keepsplit::KeepSplit;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};
pub struct Tokenizer {
    reader: BufReader<File>,
    buffer: VecDeque<String>, // push_back and pop_front
}

impl Tokenizer {
    pub fn new(path: &str) -> Self {
        Tokenizer {
            reader: BufReader::new(File::open(path).expect("File not Found")),
            buffer: VecDeque::new(),
        }
    }

    pub fn next_token(&mut self) -> Option<String> {
        let r = self.peek_token();
        assert_eq!(r, self.buffer.pop_front());
        r
    }

    pub fn peek_token(&mut self) -> Option<String> {
        if self.buffer.len() == 0 {
            loop {
                let mut line = String::new();
                let n = self.reader.read_line(&mut line).unwrap();
                //println!("n: {}", n);
                let r = self.tokenize_line(&mut line);
                if let None = r {
                    if n == 0 {
                        return None;
                    }
                } else if let Some(_) = r {
                    break;
                }
            }
        }
        return self.buffer.front().cloned();
    }

    fn tokenize_line(&mut self, line: &mut String) -> Option<()> {
        //assert_eq!(line.pop().unwrap(), '\n');
        //println!("adding {} to buffer", line);
        for word in line
            .split_whitespace()
            .flat_map(|s| s.keepsplit('('))
            .flat_map(|s| s.keepsplit(')'))
            .flat_map(|s| s.keepsplit('.'))
            .flat_map(|s| s.keepsplit('λ'))
            .flat_map(|s| s.keepsplit(';'))
            .filter(|s| !s.is_empty())
        {
            self.buffer.push_back(word.to_owned());
        }
        if self.buffer.is_empty() {
            None
        } else {
            Some(())
        }
    }
}
