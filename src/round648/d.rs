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

#[derive(PartialEq, Eq)]
enum MatrixElem {
    Empty,
    Good,
    Bad,
    Wall
}

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let cases = input.next();
    for _ in 0..cases {
        let rows: usize = input.next();
        let columns: usize = input.next();
        let mut grid = Vec::new();
        let mut grid_blocked = Vec::new();
        let mut reachable = Vec::new();
        
        let mut count_good = 0;
        for _ in 0..rows {
            let mut cur_row = Vec::new();
            let mut cur_blocked_row = Vec::new();
            let mut cur_reachable = Vec::new();
            let line: String = input.next();
            for c in line.chars() {
                if c == '.' {
                    cur_row.push(MatrixElem::Empty);
                    cur_blocked_row.push(false);
                } else if c == 'G' {
                    cur_row.push(MatrixElem::Good);
                    cur_blocked_row.push(false);
                    count_good += 1;
                } else if c == 'B' {
                    cur_row.push(MatrixElem::Bad);
                    cur_blocked_row.push(false);
                } else {
                    cur_row.push(MatrixElem::Wall);
                    cur_blocked_row.push(true);
                }

                cur_reachable.push(false);
            }

            grid.push(cur_row);
            grid_blocked.push(cur_blocked_row);
            reachable.push(cur_reachable);
        }

        for r in 0..rows {
            for c in 0..columns {
                if (r > 0 && grid[r - 1][c] == MatrixElem::Bad) || (r < (rows - 1) && grid[r + 1][c] == MatrixElem::Bad) ||
                   (c > 0 && grid[r][c - 1] == MatrixElem::Bad) || (c < (columns - 1) && grid[r][c + 1] == MatrixElem::Bad) {
                    grid_blocked[r][c] = true;
                }
            }
        }

        let mut work_queue = VecDeque::new();
        work_queue.push_back((rows - 1, columns - 1));
        let mut reachable_good = 0;

        loop {
            if work_queue.is_empty() {
                break;
            } else {
                let (r, c) = work_queue.pop_front().unwrap();
                if !reachable[r][c] && !grid_blocked[r][c] {
                    reachable[r][c] = true;
                    if grid[r][c] == MatrixElem::Good {
                        reachable_good += 1;
                    }

                    if r > 0 {
                        work_queue.push_back((r - 1, c));
                    }
    
                    if r < rows - 1 {
                        work_queue.push_back((r + 1, c));
                    }
    
                    if c > 0 {
                        work_queue.push_back((r, c - 1));
                    }

                    if c < columns - 1 {
                        work_queue.push_back((r, c + 1));
                    }
                }
            }
        }

        if reachable_good == count_good {
            writeln!(out, "Yes").unwrap();
        } else {
            writeln!(out, "No").unwrap();
        }
    }
}
