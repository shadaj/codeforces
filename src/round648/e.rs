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

    let elems = input.next();

    let mut nums_with_index: Vec<HashSet<usize>> = Vec::new();
    // let mut nums: Vec<u64> = Vec::new();
    for i in 0..elems {
        let mut cur_num: u64 = input.next();
        let mut cur_index: usize = 0;
        loop {
            if cur_num > 0 {
                loop {
                    if (nums_with_index.len() as i32) - 1 < (cur_index as i32) {
                        nums_with_index.push(HashSet::new());
                    } else {
                        break;
                    }
                }

                if cur_num & 1 == 1 {
                    nums_with_index[cur_index].insert(i);
                }

                cur_num = cur_num >> 1;
                cur_index += 1;
            } else {
                break;
            }
        }
    }

    let max_index = nums_with_index.len() - 1;
    let mut nums_with_max = nums_with_index[max_index].clone();
    let k = nums_with_max.len() + 2;
    let mut total_value = 0;
    for i in (0..(max_index + 1)).rev() {
        let mut count_already_in_set = 0;
        for elem in &nums_with_index[i] {
            if nums_with_max.contains(elem) {
                count_already_in_set += 1;
            }
        }

        if count_already_in_set >= k - 2 {
            total_value += 1 << i;
        } else {
            for elem in &nums_with_index[i] {
                if nums_with_max.len() == k {
                    break;
                } else if !nums_with_max.contains(elem) {
                    nums_with_max.insert(*elem);
                    count_already_in_set += 1;
                }
            }

            if count_already_in_set >= k - 2 {
                total_value += 1 << i;
            }
        }
    }

    writeln!(out, "{}", total_value).unwrap();
}
