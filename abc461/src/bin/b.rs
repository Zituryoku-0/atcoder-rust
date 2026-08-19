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
        a: [usize; n],
        b: [usize; n],
    }

    for i in 0..n {
        let have = a[i] - 1;
        if i != b[have] - 1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
