use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut chrs: Vec<char> = s.chars().collect();
    let mut ans = 0;

    let mut start = Vec::new();
    for i in 0..n {
        if chrs[i] == 'A' && i < s.len() - 2 {
            start.push(i);
        }
    }

    if start.len() == 0 {
        println!("0");
        return;
    }
    for i in 0..start.len() {
        if chrs[start[i] + 1] == 'B' && chrs[start[i] + 2] == 'C' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
