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

    let mut ans = n / 2;
    if n % 2 != 0 {
        ans += 1;
    }

    println!("{}", ans);
}
