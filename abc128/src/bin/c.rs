use ascii::AsciiChar::M;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;
use std::vec;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・
* 【ACできたか？】
* ・
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・N <= 10
* ・O(N^2) は可能
*
* 【Observation（問題から分かった事実）】
* ・N,M,Kが全て10のため、全探索しても十分高速である
* ・ビット全探索で、Vec[false; n]で電球が立つときにtrueにするで良さそう
* ・一番大外のforはbit全探索
・その内側には、k1~kmの各ビットが立っているスイッチの
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
    1.ビットを立てた時に、電球がついたとして扱った時に、その時点で「立てた数 % 2 == p[i]」を判定する（そのビットを表す数字が存在しない場合はスキップ）
    2.それをmまで繰り返す
*
* 【計算量】
* ・O(2^10 * 10 * 10)
*/

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut k = vec![0usize; m];
    let mut s = vec![vec![false; 10]; m];
    for i in 0..m {
        input! {
            k_input: usize,
        }
        k[i] = k_input;

        for j in 0..k[i] {
            input! {
                s_input: usize,
            }
            let s_input = s_input - 1;
            s[i][s_input] = true;
        }
    }
    input! {
        p: [usize; m],
    }

    let mut ans = 0;

    for bit in 0..(1usize << n) {
        let mut ok = true;
        for i in 0..m {
            let mut sum = 0;
            // println!("この時のbit：{}", bit);
            for j in 0..10 {
                if ((bit >> j) & 1) != 0 && s[i][j] {
                    // println!("通った時j：{}", j);
                    // println!("通った時のs[i][j]：{}", s[i][j]);
                    sum += 1;
                }
            }

            if sum % 2 != p[i] {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}
