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
        x: usize,
    }
    if x != 1 {
        println!("{}", 1);
    } else if x != 2 {
        println!("{}", 2);
    } else {
        println!("{}", 3);
    }
}
