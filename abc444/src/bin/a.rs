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
    }
    let mut chr: Vec<char> = n.to_string().chars().collect();

    if chr[0] == chr[1] && chr[1] == chr[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
