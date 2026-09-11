use num_traits::ops::checked;
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
→クエリの中で、タイプ2の時に排出したバッテリーの残量を出力する問題
*
* 【制約から分かること】
* ・N <= 3*10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・時刻t0に残量w0で挿入されたバッテリーは、
   時刻tでは

   min(w0 + (t - t0), V)
   = min(t + (w0 - t0), V)となる

   タイプ2の時刻tは全バッテリーで共通なので、
   残量の代償関係はw0 - t-の大小関係だけで決まる

   したがって、Binaryheapにはw - tを入れておけば良い

   タイプ1： w - tをpush

   タイプ2：最大のw-tをpopし、min(V, t + (w - t))を出力

   【計算量】
   各クエリO(logQ)
   全体O(QlogQ)
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
* ・Observationの通り
*
* 【計算量】
* ・O(N)
*/

fn main() {
    input! {
        q: usize,
        v: i64,
    }

    let mut heap = BinaryHeap::new();
    for i in 0..q {
        input! {
            a: usize,
        }

        if a == 1 {
            input! {
                t: i64,
                w: i64,
            }
            // w - tをヒープに追加する
            heap.push(w - t);
        } else {
            input! {
                t: i64,
            }
            // v.min(heap.pop() + t)が答えとなる

            match heap.pop() {
                Some(battery) => println!("{}", v.min(battery + t)),
                None => println!("{}", -1),
            }
        }
    }
}
