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
    let mut dp = vec![vec![0_i64; w + 1]; n + 1];
    for i in 0..n {
        let (weight, value) = items[i];
        for j in 0..=w {
            // 選ばない
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);

            // 選ぶ
            if j + weight <= w {
                dp[i + 1][j + weight] = dp[i + 1][j + weight].max(dp[i][j] + value);
            }
        }
    }
    println!("{}", dp[n][w]);
}
