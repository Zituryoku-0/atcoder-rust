use im_rc::HashMap;
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
*【問題にかかった時間（ACできたか問わない）】
・約40分
【ACできたか？】
・できなかった
*
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・Q <= 2 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
・そもそも、c = x[q] - (a[i] + b[j])なので、cの計算はしたくない(計算量としてはO(nm)で抑えれる)
・普通にx[q]を最初もらって、c = x[q] - (a[i] + b[j])があるかを計算していけば良さそう
*
* 【状態・操作の整理】
* ・何が変化するか：
* ・何が変化しないか：
* ・答えを決めるために必要な情報：
    ・x[q] - (a[i] + b[j])と等しいcが存在するか
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
・x[q] - (a[i] + b[j])と等しいcが存在するかと言い換えれる
*
* 【解法】
* ・上記まで整理すると、〜で解ける
・x[q] - (a[i] + b[j])を計算しけば良い
・最後に、cと等しいものが存在するかを確認する
*
* 【計算量】
* ・O(q + NM)
*/

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        m: usize,
        b: [i64; m],
        l: usize,
        c: [i64; l],
        q: usize,
        x: [i64; q],
    }

    let mut map = HashMap::new();
    for i in 0..l {
        map.insert(c[i], 1);
    }

    for i in 0..q {
        let mut check = false;
        for j in 0..n {
            if x[i] - a[j] < 0 {
                continue;
            }
            for k in 0..m {
                if x[i] - (a[j] + b[k]) < 0 {
                    continue;
                }
                // x[q - (a[j] + b[k])]が存在すればそのまま出力する
                if let Some(v) = map.get(&(x[i] - (a[j] + b[k]))) {
                    println!("Yes");
                    check = true;
                    break;
                }
            }
            if check {
                break;
            }
        }
        if !check {
            println!("No");
        }
    }
}
