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

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let cases = input.next();
    for _ in 0..cases {
        let num_count = input.next();
        let mut nums: Vec<u32> = Vec::new();
        for _ in 0..num_count {
            nums.push(input.next());
        }

        let mut types: Vec<u8> = Vec::new();
        for _ in 0..num_count {
            types.push(input.next());
        }

        let mut nums_type0 = Vec::new();
        let mut nums_type1 = Vec::new();
        for (idx, elem) in nums.iter().enumerate() {
            if types[idx] == 0 {
                nums_type0.push(*elem);
            } else {
                nums_type1.push(*elem);
            }
        }

        let type0_sorted = nums_type0.windows(2).all(|elems| elems[0] <= elems[1]);
        let type1_sorted = nums_type1.windows(2).all(|elems| elems[0] <= elems[1]);

        if (type0_sorted || !nums_type1.is_empty()) && (type1_sorted || !nums_type0.is_empty()) {
            writeln!(out, "Yes").unwrap();
        } else {
            writeln!(out, "No").unwrap();
        }
    }
}
