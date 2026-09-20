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
 * 典型的なDPな感じがする
 * 最終的に何を求めたい？
 * →商品数の最大値(買えたの最大値)
 *
 * 紙幣が1ドルと、Kドル
 * N個のデザート、M個のドリンク
 *
 * O(N + M)で終わらせたいが全探索
 *
 * デザートは、両方使えるけど、ドリンクがKドル紙幣しか使えないので、ドリンクを優先して買ってみる
 *
 * ソート：nlogn + mlogm
 *
 * 優先してドリンクを買いに行く
 * 全部ドリンクを買いに行った、もしくは、ドリンクの値段 / K > Kで買い物できなくなったら、デザートを買いに行く
 *
 * ドリンクとデザートのKドルの使用量を考える必要がある
 * 極端にドリンクが高いと、Kドル紙幣を無駄使いすることになる
 * デザートをまず全ての1ドル紙幣で買い物する
 * 1ドル紙幣を使い切った状態で、デザートとドリンクのKドルの使用量を比較する
 *
 * ドリンクが極端に高いと、コスパが悪くなる
 *  (b_ary[i] / k + 1) <= (one_shihei + k * using - b_ary[i]) / k ならばドリンクを買った方が良い
 * そうでないなら、デザートを買った方が良い
 *
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        x: usize,
        y: usize,
    }

    let mut a_ary: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut b_ary: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    for i in 0..n {
        input! {
            a: usize,
        }
        a_ary.push(Reverse(a));
    }

    for i in 0..m {
        input! {
            b: usize,
        }
        b_ary.push(Reverse(b));
    }

    let mut one_shihei = x;
    let mut k_shihei = y;
    let mut ans = 0;

    // まずはデザートを買い切る
    for i in 0..n {
        // Kドル紙幣の枚数 < 購入できる一番やすいドリンクの値段 / K
        let Reverse(using) = a_ary.pop().unwrap();
        // println!("1ドルのusing：{}", using);

        // 買えなかった場合は戻しておく
        if one_shihei < using {
            a_ary.push(Reverse(using));
            break;
        }

        one_shihei -= using;
        ans += 1;
    }

    // 1ドルで買える分だけ買った状態から、デザートとドリンクを比較して買う
    while let (Some(Reverse(dezert)), Some(Reverse(drink))) = (a_ary.pop(), b_ary.pop()) {
        // Kドル紙幣の枚数 < 購入できる一番やすいドリンクの値段 / K
        let using = drink / k + 1;
        //  (b_ary[i] / k + 1) <= (one_shihei + k * using - b_ary[i]) / k ならばドリンクを買った方が良い
        if using <= (one_shihei + k * using - drink) / k {
            if k_shihei < using {
                break;
            }
            // println!("kドルのusing：{}", using);

            k_shihei -= using;
            one_shihei += k * using - drink;
            ans += 1;
            // ドリンクを買った場合は、デザートを戻す
            a_ary.push(Reverse(dezert));
        } else {
            // 買えなかった場合は戻しておく
            if one_shihei < dezert {
                a_ary.push(Reverse(using));
                break;
            }

            one_shihei -= dezert;
            ans += 1;
            // デザートを買った場合は、ドリンクを戻す
            b_ary.push(Reverse(drink));
        }
    }

    while let Some(Reverse(drink)) = b_ary.pop() {
        let using = drink / k + 1;
        if k_shihei < using {
            break;
        }
        k_shihei -= drink / k + 1;
        one_shihei += k * using - drink;
        ans += 1;
    }

    while let Some(Reverse(dezart)) = a_ary.pop() {
        if one_shihei < dezart {
            a_ary.push(Reverse(dezart));
            break;
        }

        one_shihei -= dezart;
        ans += 1;
    }

    println!("{}", ans);
}
