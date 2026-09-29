use ascii::AsciiChar::L;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・約43分
* 【ACできたか？】
* ・できなかった
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・N <= 3 * 10^5
* ・O(N^2) は可能 / 不可能
*
* 【Observation（問題から分かった事実）】
* ・ディーラーは、基本的にx < A[i]のものを優先して渡すはず
* ・x < A[i]を一通り渡してから、xを加算すると必ず勝てる最小値となるはず
* ・xが全てのA[i]より大きい場合は即座に-1
* ・x == 1の場合は、即座に1
* ・A.len() >= xの場合、A.len() + 1が答えになる
* ・A.len() < xの時、累積和の内容になる
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
1.条件を満たさないティーバッグを全部加算する
1-1.昇順でAをソートしておく
1-2.累積和を出しておく
1-3.二分探索して、x < A[i]であるものの境界値を見つけておく
1-4.sums[left + 1] + (len - left - 1) * (b - 1) + 1が答えになりそう
*
* 【計算量】
* ・O(NlogN + Q + N)
*/

fn main() {
    input! {
        n: usize,
        q: usize,
        a_input: [usize; n],
    }

    let mut a = a_input.clone();
    a.sort();
    let mut sums = vec![0usize; n + 1];
    let mut max_num = *a.last().unwrap();
    let mut len = a.len();

    for i in 1..=n {
        sums[i] = sums[i - 1] + a[i - 1];
    }

    for i in 0..q {
        input! {
            b: usize,
        }

        if max_num < b {
            println!("-1");
            continue;
        }

        let mut left = 0;
        let mut right = n;

        while left < right {
            let mid = (left + right) / 2;
            if a[mid] < b {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        let idx = left;

        println!("{}", sums[idx] + (n - idx) * (b - 1) + 1);
    }
}
