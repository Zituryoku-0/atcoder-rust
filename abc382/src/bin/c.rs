use ascii::AsciiChar::L;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;
use std::usize;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・
* 【ACできたか？】
* ・
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・N <= 2 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・B[j] >= A[i]であれば、その寿司を食べるので、愚直にいくとO(NM)
* ・ヒントより、候補列Cを作る
これにより、必然的にC[i] > C[i + 1]なるので、C[i] > A[i]となるものをCに追加していき、候補列を作成する
・候補列を作ると、寿司B[i]を食べる、食べないの境界を作ることができるため、その部分を二分探索で求めていく
今回の場合、その寿司を食べてくれるをright、食べないをleftとして持つのが良さそう
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
* ・O(N + Mlogm)
*/

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut a = Vec::with_capacity(n);
    for i in 0..n {
        input! {
            input_a: usize,
        }

        if i == 0 {
            a.push((input_a, i));
            continue;
        }

        let last = a.last().unwrap();
        if last.0 > input_a {
            a.push((input_a, i));
        }
    }

    // println!("aの中身：{:?}", a);

    for i in 0..m {
        input! {
            b: usize,
        }

        let mut left: i32 = -1;
        let mut right: i32 = a.len() as i32;
        while right - left > 1 {
            let mid = ((left + right) / 2) as usize;

            if a[mid].0 <= b {
                right = mid as i32;
            } else {
                left = mid as i32;
            }
        }

        if left == -1 {
            println!("{}", a[0].1 + 1);
        } else if right == a.len() as i32 {
            println!("-1");
        } else {
            println!("{}", a[right as usize].1 + 1);
        }
    }
}
