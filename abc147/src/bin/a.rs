use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
    }

    if a1 + a2 + a3 >= 22 {
        println!("bust");
    } else {
        println!("win");
    }
}
