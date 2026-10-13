use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        if (i + 1) % 2 != 0 {
            ans += a[i];
        }
    }
    println!("{}", ans);
}
