use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・
* 【ACできたか？】
* ・
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・N <= 20
* ・O(N^2) は可能
・各おかずについて「選ぶ/選ばない」の2択を全探索すると2^N通り
・2^20 = 10^6なので、全探索可能
*
* 【Observation（問題から分かった事実）】
* ・おかずを全探索して選んでいくで十分に高速である
* ・各おかずをbit全探索する
* ・その時の体積 <= wであればmax比較をする
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
* ・O(N * N^2)
*/

fn main() {
    input! {
        n: usize,
        w: usize,
        bento: [(usize, usize);n],
    }

    let mut ans = 0;
    for bit in 0..2 ^ n {
        let mut current_w = 0;
        let mut current_oishisa = 0;
        for i in 0..n {
            if ((bit >> i) & 1) != 0 {
                current_w += bento[i].1;
                current_oishisa += bento[i].0;
            }
        }
        if current_w <= w {
            ans = ans.max(current_oishisa);
        }
    }

    println!("{}", ans);
}
