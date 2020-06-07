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

fn left_shifts_required(mut from: u64, to: u64) -> i32 {
    let mut so_far = 0;
    loop {
        if from == to {
            return so_far;
        } else if from > to {
            return -1;
        } else {
            from = from << 1;
            so_far += 1;
        }
    }
}

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let cases = input.next();
    for _ in 0..cases {
        let a = input.next();
        let b = input.next();
        let shifts = left_shifts_required(min(a, b), max(a, b));
        if shifts == -1 {
            writeln!(out, "{}", -1).unwrap();
        } else {
            let three_shifts = shifts / 3;
            let remaining_after_three = shifts % 3;
            let two_shifts = remaining_after_three / 2;
            let one_shifts = remaining_after_three % 2;
            writeln!(out, "{}", three_shifts + two_shifts + one_shifts).unwrap();
        }
    }
}
