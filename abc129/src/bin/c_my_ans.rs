use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
 * 解説
 * 今回の場合、あるi段目までに行く方法が何通りあるのかをdpしていく
 * i段目を求める場合、iに来る方法はi-1、もしくはi-2から来ることになるため、
 * i = (i-1) + (i-2)した通りがi段目の可能段数となる
 *
 * ただし、今回はaが壊れた階段のため、そこは0として計算する
 * 例えば、i-1が壊れている場合は
 * i = 0 + (i+2)となる
 *
 * 考え方のみ見て、実装はちゃんとできた
 * 注意点
 * 今回最後にMODで剰余を計算するのではなく、都度剰余で計算したものをdp[i]に入れること
 * でないと、
 *
 *
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let num = 1000000007;

    let mut dp: Vec<i128> = vec![0; n + 1];

    // 最初は何もしないという1通りがある
    dp[0] = 1;

    let mut a_idx = 0;
    // 1段目は個別に処理
    if let Some(&v) = a.first() {
        if v == 1 {
            dp[1] = 0;
        } else {
            dp[1] = dp[0];
        }
        if v == 1 && a_idx < m - 1 {
            a_idx += 1;
        }
    } else {
        dp[1] = dp[0];
    }

    for i in 2..n + 1 {
        // iがaと一致する場合は、0とする
        if let Some(&v) = a.get(a_idx) {
            if i == v {
                if a_idx < m - 1 {
                    a_idx += 1;
                }
            } else {
                dp[i] = (dp[i - 1] + dp[i - 2]) % num;
            }
        } else {
            dp[i] = (dp[i - 1] + dp[i - 2]) % num;
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[n]);
}
