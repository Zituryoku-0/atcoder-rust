use proconio::{input, marker::Chars};
use std::cmp;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    }

    let mut dp = vec![i64::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        for j in 1..=k {
            if i < j {
                break;
            }
            dp[i] = cmp::min(dp[i], dp[i - j] + (h[i] - h[i - j]).abs());
        }
    }
    println!("{}", dp[n - 1]);
}
