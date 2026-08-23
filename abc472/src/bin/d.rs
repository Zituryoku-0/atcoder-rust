use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h], // これは実質Vec<Vec<char>>である
    }

    let mut ans = 0;
    let mut safe_mas: Vec<Vec<bool>> = vec![vec![true; w]; h];
    let mut counter_safe_mas: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                safe_mas[i]
            }
        }
    }
}
