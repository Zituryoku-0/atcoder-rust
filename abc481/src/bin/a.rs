use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut ans = a + b;

    if ans <= 9 {
        println!("1");
    } else {
        println!("2");
    }
}
