use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::ops::Bound::Excluded;
use std::ops::Bound::Unbounded;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
* 【問題の目的】
* ・最終的に何を求める問題か
abs(current - a[i])の合計を求める問題
*
* 【制約から分かること】
* ・N <= 3 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・座標を順序付きで管理し、それの前後１つの要素が分かれば良い
* ・それがわかった後に、iが小さいものが優先されれば良い
* ・
* ・
*
* 【状態・操作の整理】
* ・何が変化するか：座標
* ・何が変化しないか：クッキーを拾う個数は常にn個である
                   座標の順序
* ・答えを決めるために必要な情報：
・今いる場所の前後1つの距離
・各距離のindex（どちらのiが小さいか）
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
*
* 【解法】
* ・上記まで整理すると、〜で解ける
* 　・BTreeSetを使い、rangeを用いて、その距離の左右の値を取得し、小さい方をansに追加する
    ・使い終わったら、removeで削除する
    ・BTreeSetの要素がなくなったらansを出力する
* 【計算量】
* ・O(NlogN)くらい？
*/

fn main() {
    input! {
        n: usize,
    }
    let mut set = BTreeSet::new();
    for _ in 0..n {
        input! {
            a: i64,
        }
        set.insert(a);
    }

    let mut ans: i64 = 0;
    let mut current = 0;
    loop {
        // currentの左右を取得
        let left = set.range((Unbounded, Excluded((current)))).next_back();
        let right = set.range((Excluded((current)), Unbounded)).next();

        // currentで更新する
        match (left, right) {
            (Some(l), Some(r)) => {
                let diff_left = (l - current).abs();
                let diff_right = (r - current).abs();
                if diff_left <= diff_right {
                    let l = *l;
                    ans += diff_left;
                    current = l;
                    set.remove(&l);
                } else {
                    let r = *r;
                    ans += diff_right;
                    current = r;
                    set.remove(&r);
                }
            }
            // 末尾の場合
            (Some(l), None) => {
                let l = *l;
                ans += (l - current).abs();
                current = l;
                set.remove(&l);
            }
            // 先頭の場合
            (None, Some(r)) => {
                let r = *r;
                ans += (r - current).abs();
                current = r;
                set.remove(&r);
            }
            // 要素が空の場合
            _ => break,
        }
    }

    println!("{}", ans);
}
