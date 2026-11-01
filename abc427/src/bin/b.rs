use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn digits_sum(mut x: usize) -> usize {
    let mut sum = 0;

    while x > 0 {
        sum += x % 10;
        x /= 10;
    }

    sum
}

fn main() {
    input! {
        n: usize,
    }

    let mut a = 1usize;

    for _ in 0..n - 1 {
        a += digits_sum(a);
    }

    println!("{}", a);
}
