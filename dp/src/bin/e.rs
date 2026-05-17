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
        items: [(usize, usize);n],
    }

    let max_value = 1000 * n;
    let inf = 1usize << 60;

    let mut dp = vec![inf; max_value + 1];
    dp[0] = 0;

    for (weight, value) in items {
        for v in (value..=max_value).rev() {
            dp[v] = dp[v].min(dp[v - value] + weight);
        }
    }

    let mut ans = 0;

    for v in 0..=max_value {
        if dp[v] <= w {
            ans = v;
        }
    }

    println!("{}", ans);
}
