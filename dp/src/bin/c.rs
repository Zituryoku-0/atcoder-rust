use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        event: [[i32; 3];n],
    }

    let mut dp = vec![vec![0; 3]; n];

    for i in 0..n {
        for j in 0..3 {
            if i == 0 {
                dp[i][j] = event[i][j];
                continue;
            }
            for k in 0..3 {
                // 前日と同じものは選べない
                if j == k {
                    continue;
                }
                dp[i][j] = dp[i][j].max(dp[i - 1][k] + event[i][j]);
            }
        }
    }

    let mut ans = 0;
    for i in 0..3 {
        if ans < dp[n - 1][i] {
            ans = dp[n - 1][i];
        }
    }
    println!("{}", ans);
}
