use im_rc::HashMap;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
 * mapで0人以上のクラスを管理するのはあり
 * てかそれで良くね？
 * value比較して1個でも多いものがあれば悲しむで終わり
 *
 * value比較の方法
 * mapをvalueで配列で戻す
 * 降順でソートする
 * 先頭の値と-1までであればansに入れる
 * -2あるなら、処理を終わる
 *
 * そもそもmapで管理すべきでない
 * 理由は、すべてのクラスの人数が1人だった時に、ansがkになるから
 * 特例入れればギリ耐え？
 */

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i32; n],
    }

    let mut map = HashMap::new();

    for i in 0..n {
        // (クラス, 人数) (key, value)
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ary: Vec<i32> = map.values().copied().collect();
    // println!("map：{:?}", ary);

    ary.sort();
    let max_v = ary[ary.len() - 1];
    let mut ans = 1;

    // クラスが1つしかなければ終わり
    if ary.len() == 1 {
        println!("{}", ans);
        return;
    }

    // 全クラスが1人だった場合は、kが答え
    if max_v == 1 {
        println!("{}", k);
        return;
    }

    for i in (0..ary.len() - 1).rev() {
        // println!("最大所属人数：{}", max_v);
        // println!("比較人数：{}", ary[i]);
        // 差分が1までなら悲しまない
        if max_v - ary[i] < 2 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
