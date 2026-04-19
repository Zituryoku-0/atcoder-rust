use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        grid: [Chars; r], // これは実質Vec<Vec<char>>である
        arrays: [i64; n],
        multi_arrays: [(i64, i64); n]
    }
    print!("{}", n);
    print!("{:?}", arrays);
    print!("{:?}", multi_arrays);
}
