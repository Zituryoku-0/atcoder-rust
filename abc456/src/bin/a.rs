use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        x: usize,
    }

    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                if x == (i + 1) + (j + 1) + (k + 1) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
