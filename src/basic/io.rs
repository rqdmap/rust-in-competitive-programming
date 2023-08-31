use std::io::prelude::*;
pub fn stdin_to_string() -> String {
    let mut content = String::new();
    let stdin = std::io::stdin();
    let mut rd = stdin.lock();
    rd.read_to_string(&mut content).unwrap();
    return content;
}

pub struct Parser<'a> {
    tokens: std::str::SplitWhitespace<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            tokens: text.split_whitespace(),
        }
    }

    pub fn read_or_eof<T: std::str::FromStr>(&mut self) -> Option<T> {
        self.next().map(|s| match s.parse() {
            Ok(x) => x,
            Err(_) => panic!("cannot parse {:?}", s),
        })
    }

    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        self.read_or_eof::<T>().expect("unexpected end-of-file")
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        self.tokens.next()
    }
}

#[macro_export]
macro_rules! input {
    ($($i:expr), +) => {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        $($i = iter.next().unwrap().parse().unwrap();)*
    };

    ($vec:ident ; $n:expr) => {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        $vec = buf.split_whitespace()
            .map(|x| x.parse().unwrap())
            .take($n)
            .collect();
    };
}
