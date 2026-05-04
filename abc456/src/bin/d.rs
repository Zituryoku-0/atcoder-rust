use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        s: String,
    }
    const MOD: i64 = 998244353;

    let mut dp = vec![0_i64; 3];
    let mut total = 0_i64;

    for ch in s.chars() {
        let idx = match ch {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => unreachable!(),
        };

        let add = (1 + total - dp[idx] + MOD) % MOD;
        dp[idx] += add;
        dp[idx] %= MOD;

        total += add;
        total %= MOD;
    }

    println!("{}", total);
}
