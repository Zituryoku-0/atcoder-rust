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
 * 解説
 * まずドリンクを全て買い、残ったお金でデザートを買えば良い
 * ここでのポイントは、ドリンクを全探索する、デザートは累積和を二分探索することで、計算量を抑えることができる
 * o(m) + o(logn) + o(n)
 *
 * 全体のアルゴリズム
 * 1.A（ドリンク）、B（デザート）を昇順ソートする
 * 2.Aの価格の累積和を作る
 * 3.Bの価格の累積和を作る
 * 4.各B[i]に必要なKドル紙紙幣を計算する
 * 5.その累積和も作る
 * 6.ドリンクを0..=m個買う場合を全探索する
 * 7.Kドル紙幣が足りるか確認
 * 8.残りの総額を計算
 * 9.デザートを何個買えるか二分探索
 * 10.デザート数 + ドリンク数の最大値を更新
 *
 *
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        x: i64,
        y: i64,
        mut a: [i64; n],
        mut b: [i64; m],
    }

    // 安い商品から選べるようにソート
    a.sort();
    b.sort();

    // Aの価格の累積和
    // prefix_a[i] = Aの安い方からi個買った時の合計価格
    let mut prefix_a = vec![0_i64; n + 1];

    for i in 0..n {
        prefix_a[i + 1] = prefix_a[i] + a[i];
    }

    // Bの価格の累積和
    let mut prefix_b = vec![0_i64; m + 1];

    // Bを買うために必要なKドル紙幣の累積和
    let mut prefix_k = vec![0_i64; m + 1];

    for i in 0..m {
        prefix_b[i + 1] = prefix_b[i] + b[i];

        // ceil(b[k] / k)
        let required_k = (b[i] - 1) / k + 1;

        prefix_k[i + 1] = prefix_k[i] + required_k;
    }

    // 手持ちのお金の総額
    let total_money = x + k * y;

    let mut ans = 0_usize;

    // ドリンクをdrink_count個買う場合を全探索
    for drink_count in 0..=m {
        // Kドル紙幣が足りない
        if prefix_k[drink_count] > y {
            break;
        }

        // ドリンクを買った後の残金
        let remain = total_money - prefix_b[drink_count];

        if remain < 0 {
            break;
        }

        // prefix_a[dessert_count] <= remain
        // となる最大のdessert_countを二分探索
        let mut ok = 0_usize;
        let mut ng = n + 1;

        while ng - ok > 1 {
            let mid = (ok + ng) / 2;

            if prefix_a[mid] <= remain {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        let dessert_count = ok;

        ans = ans.max(drink_count + dessert_count);
    }
    println!("{}", ans);
}
