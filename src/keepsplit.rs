use std::str::Split;

pub trait KeepSplit {
    fn keepsplit(&self, pat: char) -> Vec<String>;
}

impl<T> KeepSplit for T
where
    T: AsRef<str>,
{
    fn keepsplit(&self, pat: char) -> Vec<String> {
        let mut n = Vec::<String>::new();

        for s in self.as_ref().split(pat) {
            n.push(s.to_string());
            n.push(pat.to_string());
        }
        n.pop();

        n
    }
}

/*


[a, b, c, d] -> [a, ., b, ., c, ., d]

*/
