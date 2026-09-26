use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: usize,
        grid: [Chars; n], // これは実質Vec<Vec<char>>である
        arrays: [i64; n],
        multi_arrays: [(i64, i64); n]
    }
}
