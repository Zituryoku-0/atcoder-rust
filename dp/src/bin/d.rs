use im_rc::hashmap::Values;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        w: usize,
        items: [(usize, i64);n],
    }
    let mut dp = vec![0_i64; w + 1];

    for (weight, value) in items {
        for j in (weight..=w).rev() {
            dp[j] = dp[j].max(dp[j - weight] + value);
        }
    }

    println!("{}", dp[w]);
}
