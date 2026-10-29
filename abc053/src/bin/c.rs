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
* ・N <= 10^15
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・貪欲にその中で最大の目になるように返せば良い（基本的に6,5,6,5を繰り返す）
* ・11での繰り上げ除算にした時の剰余を6,5でそれぞれ回数を1回分追加する
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
* ・O(...)
*/

fn main() {
    input! {
        x: usize,
    }

    let mut ans = 0;
    let mut temp = x % 11;
    ans += 2 * (x / 11);
    if temp == 0 {
    } else if temp > 6 {
        ans += 2;
    } else {
        ans += 1;
    }

    // println!("149696127901 / 11の値：{}", 149696127901 as usize / 11);
    // println!("149696127901 % 11の値：{}", 149696127901 as usize % 11);

    println!("{}", ans);
}
