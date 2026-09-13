use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::print;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        s: String,
    }
    let mut chr: Vec<char> = s.chars().collect();

    for i in 0..chr.len() {
        print!("{}", chr[i]);
        if i < chr.len() - 1 {
            print!("o");
        }
    }
}
