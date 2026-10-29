use nalgebra::zero;
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
* ・N <= 1000
* ・O(N^2) は可能
*
* 【Observation（問題から分かった事実）】
* ・0の連結回数を気にする必要がある
* ・0の連結回数を管理するVecを持って、その時の連結回数で「00」を使うことで簡略化する
* ・それ以外は長さ文になるため、0以外の数字の長さ + 「0」と「00」の回数の合計を求めれば良い
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
* ・O(...)
*/
fn main() {
    input! {
        s: String,
    }

    let chrs: Vec<char> = s.chars().collect();
    let mut ans = 0;
    let mut zeros = Vec::new();

    let mut renketu = 0;
    let mut diff = 0;
    let len = chrs.len();
    for i in 0..len {
        if chrs[i] == '0' {
            renketu += 1;
            diff += 1;
        } else if renketu > 0 && chrs[i] != '0' {
            zeros.push(renketu);
            renketu = 0;
        }
    }
    if renketu > 0 {
        zeros.push(renketu);
    }

    // println!("zerosの中身：{:?}", zeros);

    ans = len - diff;

    for i in 0..zeros.len() {
        if zeros[i] > 1 {
            if zeros[i] % 2 == 0 {
                ans += zeros[i] / 2;
            } else {
                ans += zeros[i] / 2 + 1;
            }
        } else {
            ans += 1;
        }
    }
    println!("{}", ans);
}
