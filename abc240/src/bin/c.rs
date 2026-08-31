use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    // 到達可能性のdpを作成すれば良い
    // i回数、jその時に可能な距離

    let mut dp = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        input! {
            a:usize,
            b:usize,
        }
        for j in 0..x {
            if !dp[i - 1][j] {
                continue;
            }

            if j + a <= x {
                dp[i][j + a] = true;
            }

            if j + b <= x {
                dp[i][j + b] = true;
            }
        }
    }

    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
