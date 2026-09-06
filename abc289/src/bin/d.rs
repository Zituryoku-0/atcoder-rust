use ascii::AsciiChar::X;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
 * 単純にX段目に登れるかを判定できれば良い
 * なぜなら、何回目の時点でX段目に登ることはできるかといった問題ではないからである
 * dp[i]で管理する
 */

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    }

    let mut dp = vec![false; x + 1];

    let mut blocked = vec![false; x + 1];

    // もちのある場所をあらかじめ設定しておく
    for i in 0..m {
        blocked[b[i]] = true;
    }

    dp[0] = true;

    for i in 0..x {
        // 現時点で到達不能もしくは、もちがある場合はスキップ
        if !dp[i] || blocked[i] {
            continue;
        }
        for j in 0..n {
            if i + a[j] > x {
                break;
            }
            dp[i + a[j]] = true;
        }

        // X段目に行けたか
        if dp[x] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
