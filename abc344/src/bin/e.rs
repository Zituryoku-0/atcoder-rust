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
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・N <= 2 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・xの後ろにyを挿入するのは、x（A[i]）のnextがyになり、yのnextがA[i+1]になる
もしxが一番後ろの場合はnextがNoneである
* ・xを削除するとき、yのprevがA[i-1]になる
もしxが一番前の場合はprevがNoneである
* ・だからこそ、問題文でA[i]!= A[j]であることを保証しているのか
*
* 【状態・操作の整理】
* ・何が変化するか：x、yの前後関係
* ・何が変化しないか：
* ・答えを決めるために必要な情報：
・全てのクエリを実行した後のグラフが分かれば良い
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
・各クエリを実行した後のグラフを出力せよと考え直せる

* 【解法】
* ・上記まで整理すると、〜で解ける
・各クエリを実行する際のxのインデックス（愚直にいくなら、先頭から見るO(n)）
→もう少し楽に見つけることができれば問題なさそう
*
* 【計算量】
* ・O(nq)微妙だけど一旦実装してみる
*/

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut next: HashMap<usize, Option<usize>> = HashMap::new();
    let mut prev: HashMap<usize, Option<usize>> = HashMap::new();

    // 先頭
    let mut front = a[0];

    // 初期状態の双方向連結リストを作る
    for i in 0..n {
        let p = if i == 0 { None } else { Some(a[i - 1]) };

        let nxt = if i + 1 == n { None } else { Some(a[i + 1]) };
        prev.insert(a[i], p);
        next.insert(a[i], nxt);
    }

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: usize,
                y: usize,
            }

            // xの現在の次の要素
            let old_next = next[&x];

            // x -> y
            next.insert(x, Some(y));
            prev.insert(y, Some(x));

            // y -> old_next
            next.insert(y, old_next);

            // old_nextが存在するなら
            // old_next.prev = y
            if let Some(nxt) = old_next {
                prev.insert(nxt, Some(y));
            }
        } else {
            input! {
                x: usize,
            }

            let p = prev[&x];
            let nxt = next[&x];

            // xの前に要素がある
            if let Some(p) = p {
                next.insert(p, nxt);
            } else {
                // xが先頭だった
                front = nxt.unwrap();
            }

            // xの後ろに要素がある
            if let Some(nxt) = nxt {
                prev.insert(nxt, p);
            }
        }
    }

    // 先頭からnextを辿って出力
    let mut current = Some(front);
    let mut ans = Vec::new();

    while let Some(x) = current {
        ans.push(x);
        current = next[&x];
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
