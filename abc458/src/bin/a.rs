use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        s: String,
        n: usize,
    }

    let mut chr: Vec<char> = s.chars().collect();
    for i in n..chr.len() - n {
        print!("{}", chr[i]);
    }

    println!();
}
