use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::print;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        s: String,
    }
    let mut chrs: Vec<char> = s.chars().collect();

    for i in 0..chrs.len() {
        if chrs[i] == 'A' {
            print!("A");
        } else {
            print!(".");
        }
    }
    println!();
}
