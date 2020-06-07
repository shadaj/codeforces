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
    let empty_vec: Vec<u32> = Vec::new();
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let nodes: u32 = input.next();
    let references: u32 = input.next();

    let mut adjacency = HashMap::new();

    for _ in 0..references {
        let first: u32 = input.next();
        let second: u32 = input.next();
        
        if !adjacency.contains_key(&first) {
            adjacency.insert(first, Vec::new());
        }

        if !adjacency.contains_key(&second) {
            adjacency.insert(second, Vec::new());
        }

        adjacency.get_mut(&first).unwrap().push(second);
        adjacency.get_mut(&second).unwrap().push(first);
    }

    let mut topic_to_nodes = HashMap::new();
    let mut node_to_topic = HashMap::new();
    for i in 0..nodes {
        let node = i + 1;
        let topic: u32 = input.next();
        node_to_topic.insert(node, topic);

        if !topic_to_nodes.contains_key(&topic) {
            topic_to_nodes.insert(topic, Vec::new());
        }
        topic_to_nodes.get_mut(&topic).unwrap().push(node);
    }

    let mut node_to_min_topic_option = HashMap::<u32, u32>::new();
    let mut nodes_order = Vec::new();
    
    let check_all = (1..(nodes + 1)).all(|topic| {
        topic_to_nodes.get(&topic).unwrap_or(&empty_vec).iter().all(|node| {
            nodes_order.push(node);
            if *node_to_min_topic_option.get(node).unwrap_or(&1) == topic {
                adjacency.get(node).unwrap_or(&Vec::new()).iter().all(|neighbor| {
                    if *node_to_topic.get(neighbor).unwrap() != topic {
                        if *node_to_min_topic_option.get(neighbor).unwrap_or(&1) == topic {
                            node_to_min_topic_option.insert(*neighbor, topic + 1);
                        }

                        true
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        })
    });

    if check_all {
        for elem in nodes_order {
            write!(out, "{} ", elem).unwrap();
        }

        writeln!(out).unwrap();
    } else {
        writeln!(out, "{}", -1).unwrap();
    }
}
