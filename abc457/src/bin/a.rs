use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }

    println!("{}", a[x - 1]);
}
