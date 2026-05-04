use num::abs;
use proconio::{input, marker::Chars};
use std::cmp;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    // 各足場での重み
    let mut dp = vec![0; n as usize];

    for i in 0..n {
        if i == 0 {
            continue;
        }
        if i == 1 {
            dp[i] = dp[i - 1] + (h[i] - h[i - 1]).abs();
            continue;
        }
        dp[i] = cmp::min(
            (dp[i - 1] + (h[i] - h[i - 1]).abs()),
            (dp[i - 2] + (h[i] - h[i - 2]).abs()),
        );
    }

    println!("{}", dp[n - 1]);
}
