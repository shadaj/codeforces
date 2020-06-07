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
        let rows = input.next();
        let columns = input.next();
        let mut matrix: Vec<Vec<bool>> = Vec::new();
        for _ in 0..rows {
          let mut cur_row = Vec::new();
          for _ in 0..columns {
            cur_row.push(input.next::<i8>() == 1);
          }

          matrix.push(cur_row);
        }

        let mut count_open_cols = 0;
        for c in 0..columns {
          if (0..rows).all(|r| matrix[r][c] == false) {
            count_open_cols += 1;
          }
        }

        let mut count_open_rows = 0;
        for r in 0..rows {
          if (0..columns).all(|c| matrix[r][c] == false) {
            count_open_rows += 1;
          }
        }

        
        let mut cur_ashish = true;
        loop {
          if count_open_cols == 0 || count_open_rows == 0 {
            if cur_ashish {
              writeln!(out, "Vivek").unwrap();
            } else {
              writeln!(out, "Ashish").unwrap();
            }

            break;
          } else {
            count_open_cols -= 1;
            count_open_rows -= 1;
          }

          cur_ashish = !cur_ashish;
        }
    }
}
