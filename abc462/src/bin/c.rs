use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        grid: [Chars; n], // これは実質Vec<Vec<char>>である
        arrays: [i64; n],
        multi_arrays: [(i64, i64); n]
    }
    print!("{}", n);
    print!("{:?}", arrays);
    print!("{:?}", multi_arrays);
}
