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

use std::collections::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let num_count = input.next();
    let mut a_elem_to_index: HashMap<u32, u32> = HashMap::new();
    for i in 0..num_count {
        a_elem_to_index.insert(input.next(), i);
    }

    let mut b_offsets_histogram: HashMap<i32, u32> = HashMap::new();
    for i in 0..num_count {
        let b_elem = input.next();
        let mut index_diff = (i as i32) - (*a_elem_to_index.get(&b_elem).unwrap() as i32);
        if index_diff < 0 {
            index_diff += num_count as i32;
        }

        b_offsets_histogram.insert(index_diff, b_offsets_histogram.get(&index_diff).unwrap_or(&0) + 1);
    }

    let mut best_count = 0;
    for shift in b_offsets_histogram.keys() {
        best_count = max(best_count, *b_offsets_histogram.get(shift).unwrap());
    }

    writeln!(out, "{}", best_count).unwrap();
}
