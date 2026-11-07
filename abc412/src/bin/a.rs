use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;

    for i in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        if a < b {
            ans += 1;
        }
    }
    println!("{}", ans);
}
