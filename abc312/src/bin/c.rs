use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 * 解説
 * この問題では、
 * A[i] <= Xを満たす売り手の人数
 * B[i] >= Xを満たす買い手の人数について
 * 売り手の人数 >= 買い手の人数となる最小の価格Xを求めたい
 *
 * Xが大きくなると
 * seller(X)→増える
 * buyer(X)→減るという単調性がある
 *
 * したがってseller_count >= buyer_countもNGとOKの境界が見えてくる
 * ここを二分探索するのがポイント
 *
 * 計算量
 * 各判定 O(n + m)
 * 二分探索約30回
 * O((n + m) log 10^9)
 */

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut left = 0usize; // NG
    let mut right = 1_000_000_001usize;

    while right - left > 1 {
        let mid = (left + right) / 2;

        // mid円で売っても良い売り手
        let seller_count = a.iter().filter(|&&price| price <= mid).count();

        // mid円で買っても良い買い手
        let buyer_count = b.iter().filter(|&&price| mid <= price).count();

        if seller_count >= buyer_count {
            // 条件を満たす
            // もっと小さい価格を探す
            right = mid;
        } else {
            // 条件を満たさない
            // もっと高い価格にする
            left = mid;
        }
    }

    println!("{}", right);
}
