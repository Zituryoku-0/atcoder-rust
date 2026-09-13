use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::print;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・N <= 5 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・クエリ2の時は、idx = 1の場所を出力する
* ・クエリ2のような処理を2回行うと、最初と値が戻る
* ・クエリ1の入れ替えは、O(1)でできる
* ・クエリ2はo(n)かかる
* ・クエリ2をo(n)かけないようにする為に、インデックスを管理する用のvecを準備するのもあり
* ・
*
* 【答えを決めるために必要な情報】
・クエリを全て処理した後の場所管理用のidxsを出力せよ
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
*
* 【解法】
* ・上記まで整理すると、〜で解ける
・最初の1をインプット時に、場所を管理する用のvecにも詰める
・クエリ1のときは、temp,a,bで入れ替え、場所管理用のvecも更新する
・クエリ2のときは、場所管理用のvecの内容で出力する
例：2,3,4,1,5で、値1の場所は4個目なので、4というようになる
*
* 【計算量】
* ・O(NQ（Qは2の数次第）)
*/

fn main() {
    input! {
        n: usize,
        q: usize,
        mut p: [usize; n],
    }

    // 全て0-indexedにする
    for x in &mut p {
        *x -= 1;
    }

    // inv[value] = valueが存在する位置
    let mut inv = vec![0; n];

    for i in 0..n {
        inv[p[i]] = i;
    }

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                x:usize,
                y:usize,
            }

            let x = x - 1;
            let y = y - 1;

            // swap前の位置
            let px = p[x];
            let py = p[y];

            // pを更新
            p.swap(x, y);

            // P^{-1}を更新
            inv[px] = y;
            inv[py] = x;
        } else {
            //P <- P^{-1}
            std::mem::swap(&mut p, &mut inv);
        }
    }

    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", p[i] + 1);
    }
    println!();
}
