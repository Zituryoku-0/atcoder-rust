use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/***
 * 解説
 * Nは最大8なので、各竹に4通りの選択肢があっても
 * 4^8 = 65536通りしかないため、全探索で十分に間に合う
 * この問題で大事なのは、
 * ・どの竹をAに使うか
 * ・どの竹をBに使うか
 * ・どの竹をCに使うか
 * を決めて仕舞えば、必要MPは簡単に計算できるという点
 *
 * 例えば、現在見ている竹が40だとする
 * この竹について、
 * 1.Aを作るために使う
 * 2.Bを作るために使う
 * 3.Cを作るために使う
 * 4.使わない
 * の4通りがある
 *
 * 今回のDFSで必要な情報は以下
 * index（今何本目の竹を見ているか）、a_sum（A用として選んだ竹の合計）, b_sum, c_sum, a_count（A用として何本使ったか）, b_count, c_count
 *
 * 竹を合算する場合、一般化すると
 * (k-1) * 10 MPポイントを消費する
 *
 * A,B,Cは、それぞれ最低1本は竹を使う必要がある
 * そのためいずれかに合致する場合、その組み合わせは不正である
 * ・A用の竹が0本
 * ・B用の竹が0本
 * ・C用の竹が0本
 * そのため、DFSの終端でA,B,Cが全部1本以上あるかをチェックする必要がある
 *
 *
 */

const INF: i32 = i32::MAX;

fn dfs(
    index: usize,
    a_sum: i32,
    b_sum: i32,
    c_sum: i32,
    a_count: i32,
    b_count: i32,
    c_count: i32,
    l: &[i32],
    a: i32,
    b: i32,
    c: i32,
) -> i32 {
    // lの最後までいったら、各コストの最小値を取得する
    if index == l.len() {
        // 組み合わせがない場合infを返す
        if a_count == 0 || b_count == 0 || c_count == 0 {
            return INF;
        }

        let length_cost = (a - a_sum).abs() + (b - b_sum).abs() + (c - c_sum).abs();

        // 結合コスト
        let combine_cost = (a_count - 1) * 10 + (b_count - 1) * 10 + (c_count - 1) * 10;

        return length_cost + combine_cost;
    }

    let current = l[index];
    // 何もしない（indexだけ進める）
    let unused = dfs(
        index + 1,
        a_sum,
        b_sum,
        c_sum,
        a_count,
        b_count,
        c_count,
        l,
        a,
        b,
        c,
    );

    let a_used = dfs(
        index + 1,
        a_sum + current,
        b_sum,
        c_sum,
        a_count + 1,
        b_count,
        c_count,
        l,
        a,
        b,
        c,
    );

    let b_used = dfs(
        index + 1,
        a_sum,
        b_sum + current,
        c_sum,
        a_count,
        b_count + 1,
        c_count,
        l,
        a,
        b,
        c,
    );

    let c_used = dfs(
        index + 1,
        a_sum,
        b_sum,
        c_sum + current,
        a_count,
        b_count,
        c_count + 1,
        l,
        a,
        b,
        c,
    );

    unused.min(a_used).min(b_used).min(c_used)
}

fn main() {
    input! {
        n: usize,
        a: i32,
        b: i32,
        c: i32,
        l: [i32; n],
    }

    let mut index = 0;
    let mut a_sum = 0;
    let mut b_sum = 0;
    let mut c_sum = 0;
    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;

    let ans = dfs(
        index, a_sum, b_sum, c_sum, a_count, b_count, c_count, &l, a, b, c,
    );

    println!("{}", ans);
}
