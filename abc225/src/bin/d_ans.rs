use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::print;
use std::println;
use std::writeln;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
* 【問題の目的】
* ・最終的に何を求める問題か
*　→クエリでC == 3の時に、xに含まれる連結成分を先頭から順に全て出力する
*
* 【制約から分かること】
* ・N <= 10^5,Q <= 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・xを管理する配列を作ったとすると、そこには後ろにyが入る（それぞれ1つずつ）
* ・yを管理する配列を作ったとすると、そこには後ろにxが入る（それぞれ1つずつ）
* ・xを含むグループの先頭が分かれば、あとはx[i]の内容をもとに次のインデックスに値が存在するまで出力すれば良い
* ・
*
* 【状態・操作の整理】
* ・何が変化するか：電車の前部と後部、連結グループ
* ・何が変化しないか：
* ・答えを決めるために必要な情報：
・xを含むグループの先頭の要素
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
   ・各電車を頂点、前後の連結を辺として双方向のグラフである
   ・双方向のグラフの先頭から出力せよとなる
*
* 【解法】
・双方向グラフを表す方法として、xの配列、yの配列をそれぞれ準備する
・もし、2を含むグループを出力しろとなった場合、
・配列yの中に、値2を持つ要素がなければ2を先頭として扱える
・配列xの中に、インデックス2に要素を持っていれば、それが後続の値として出力し、
かつそのインデックスに値があればそれを続けて出力するのを繰り返せば良い
懸念としては、入力例1のように、[6,3,5,2,7]の時に2を含むとなった場合、先頭までを見つけるのにO(n)かかる
・O(NQ)はTLEする、Qはクエリの数で減らし用がないので、Nの部分をどうにか工夫したい
・配列xに値があるが、配列yに値がない場合、その要素はグループの先頭である
・配列yに値があるが、配列xに値がない場合、その要素はグループの最後部である
・少なからず、配列yに値がなければ、何かしらのグループの先頭ではあることになる
まずは、配列yの値がないものから順に見ていくのもあり？

【重要なObservation】
・各電車の前後には最大1台しか接続されない
→ 各頂点について prev / next の2つだけ管理すればよい

・3 x では、全電車から先頭を探す必要はない
→ xからprevを辿り、prevが存在しないところが先頭

・先頭が分かったらnextを辿れば順番通り取得できる

・1回のクエリ3はO(連結成分サイズ)だが、
  出力される電車数の総和が10^6以下
→ 全体として十分高速

「各要素の接続先が高々1つなら、グラフの隣接リストではなく prev / next で直接持てないか？」
*
* 【計算量】
* ・O(NQ) めっちゃTLEしそう
*/

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut next = vec![None; n];
    let mut prev = vec![None; n];

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for i in 0..q {
        input! {
            c: usize,
        }

        // 電車を連結する
        if c == 1 {
            input! {
                x: usize,
                y: usize,
            }
            let x = x - 1;
            let y = y - 1;
            next[(x) as usize] = Some(y);
            prev[(y) as usize] = Some(x);
        } else if c == 2 {
            input! {
                x: usize,
                y: usize,
            }
            let x = x - 1;
            let y = y - 1;
            // 電車の切り離し
            next[(x) as usize] = None;
            prev[(y) as usize] = None;
        } else {
            input! {
                x: usize,
            }
            // 出力
            let mut current = x - 1;
            while let Some(v) = prev[current] {
                current = v;
            }

            // 先頭から後ろへたどる
            let mut trains = Vec::new();
            loop {
                trains.push(current);

                match next[current] {
                    Some(nxt) => current = nxt,
                    None => break,
                }
            }
            write!(out, "{}", trains.len()).unwrap();
            for train in trains {
                // write!(out, "{ }").unwrap();
                write!(out, " {}", train + 1).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
