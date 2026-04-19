use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        m: i32,
        d: i32
    }

    let gothec = vec![(1, 7), (3, 3), (5, 5), (7, 7), (9, 9)];

    for thec in gothec {
        if (m, d) == thec {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
