use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    println!("{}", r - l + 1);
}
