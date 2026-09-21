use proconio::{input, marker::Chars};
use rand_core::le;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;
use std::usize;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・約32分
* 【ACできたか？】
* ・できなかった（一個だけWA）
* 【問題の目的】
* ・最終的に何を求める問題か
   ・X >= A * N + B * d(N)を満たす整数Nを求める
*
* 【制約から分かること】
* ・N <= 10 * 8
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・Nは最大でも10^8くらい（A <= 10^9のため）
* ・X >= AN * Bd(N)を二分探索で求めれば良い
* ・
*
* 【状態・操作の整理】
* ・何が変化するか：
* ・何が変化しないか：
* ・答えを決めるために必要な情報：
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
*
* 【解法】
* ・上記まで整理すると、〜で解ける
*
* 【計算量】
* ・O(logX)
*/

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    }

    let mut left = 0;
    let mut right = 1_000_000_001;

    while right - left > 1 {
        let mid = (right + left) / 2;
        let mut temp_mid = mid;
        let mut keta = 0;
        // println!("temp_midの値：{}", temp_mid);
        while temp_mid > 0 {
            keta += 1;
            temp_mid /= 10;
        }
        // println!("a * mid + b * ketaの値{}", a * mid + b * keta);

        if x >= a * mid + b * keta {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
