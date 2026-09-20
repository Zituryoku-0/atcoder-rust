use ascii::AsciiChar::Hash;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・約30分
* 【ACできたか？】
* ・できた
* 【問題の目的】
* ・最終的に何を求める問題か
・ソートしたaについて、1以上K以下で一度も現れないものの総話を求める問題
*
* 【制約から分かること】
* ・N <= 2 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
・1からkまでを加算したものと、a[0]~a[n-1]までを加算したものを引けば良い
*
* 【状態・操作の整理】
* ・何が変化するか：
* ・何が変化しないか：
* ・答えを決めるために必要な情報：
    ・1からkまでの総和
    ・ガウスの法則より、O(1)で求めれる(1 + 1_000_000_000) * (1_000_000_000 / 2)
    ・a[0]~a[n-1]の総和
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
*
* 【解法】
* ・上記まで整理すると、〜で解ける
・aからkまでの総和
・a[0]からa[n-1]までの総和
*
* 【計算量】
* ・O(n)
*/

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut set = HashSet::new();
    let mut a = a.clone();
    a.sort();

    let k_total = if k % 2 == 0 {
        (1 + k) * (k / 2)
    } else {
        k * ((k - 1) / 2) + k
    };

    let mut a_total = 0;
    for i in 0..n {
        if a[i] <= k {
            if !set.contains(&a[i]) {
                set.insert(a[i]);
                a_total += a[i];
            }
        } else {
            break;
        }
    }

    // println!("k_total：{}", k_total);
    // println!("a_total：{}", a_total);

    println!("{}", k_total - a_total);
}
