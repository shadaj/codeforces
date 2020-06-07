// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{stdin, stdout, BufWriter, Write};

struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { buffer: Vec::new() }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

// END UTIL

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let cases = input.next();
    for _ in 0..cases {
        let count = input.next();
        let nums = (0..count).map(|_| input.next()).collect::<Vec<i32>>();
        let nums_set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());

        let max_k = (1..(1024 * 2)).find(|k| {
            for elem in &nums {
                if !nums_set.contains(&(elem ^ k)) {
                    return false;
                }
            }

            return true;
        });

        writeln!(out, "{}", max_k.unwrap_or(-1)).unwrap();
    }
}
