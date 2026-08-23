use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        x: char,
        s: [Chars; n], // これは実質Vec<Vec<char>>である

    }

    let chrs = ['A', 'B', 'C', 'D', 'E'];
    let idx = chrs.iter().position(|&chr| chr == x).unwrap();

    for i in 0..n {
        if s[i][idx] == 'o' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
