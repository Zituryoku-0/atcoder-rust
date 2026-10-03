use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 * 全探索する
 * 前から3つ選んで、総和より大きいならスキップ
 * 総和以下ならば、それをmaxで更新する
 *
 */

fn main() {
    input! {
        n: usize,
        v: usize,
        w: [usize; n],
    }

    let mut ans = 0;

    for i in 0..n {
        for j in i..n {
            for k in j..n {
                // println!("kのインデックス：{}", k);
                // 価格がvより大きいならスキップ
                if i == j || j == k || k == i || (i + j + k) + 3 > v {
                    continue;
                }

                // println!(
                //     "更新前のans：{}, i:{}, j:{}, k:{}",
                //     w[i] + w[j] + w[k],
                //     i,
                //     j,
                //     k
                // );
                ans = ans.max(w[i] + w[j] + w[k]);
            }
        }
    }

    println!("{}", ans);
}
