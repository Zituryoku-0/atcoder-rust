use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 * 解説
 * 知りたいのはkがこの範囲に入っているかだけのため、
 * let low = a[i-1].min(a[i]);
 * let high = a[i-1].max(a[i]);
 *
 * if low <= k && k <= high {
 *  // y = kと交わる
 * }
 * だけで判定できる
 * 以下は競プロではかなり重要な考え方である
 * ・区間内に特定の値が存在するか調べたいだけなら、区間を列挙しない.
 *
 * 共有端点をどう処理するか？
 * 1.A_i == Kになっている頂点を数える
 * 2.線分の内部でKを跨ぐ場合を数える
 *
 * 例えば A = [1, 3, 1, 5, 1]
 * K = 3の時
 *
 * 頂点
 * 1
 * 3　←Kなので+1
 * 1
 * 5
 * 1
 *
 * 線分の内部
 * 1 → 3 ：終点なので数えない
 * 3 → 1 ：始点なので数えない
 * 1 → 5 ：途中で3を通る→ +1
 * 5 → 1 ：途中で3を通る→ +1
 *
 * 合計
 * 1 + 1 + 1 = 3となる
 *
 * ポイントは、頂点だけで数え、その後跨ぐものを数えれば良い
 *
 *
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    // y = Kの頂点
    for &x in &a {
        if x == k {
            ans += 1;
        }
    }

    // 線分の内部でy = Kを通過するケース
    for i in 1..n {
        if (a[i - 1] < k && k < a[i]) || (a[i] < k && k < a[i - 1]) {
            ans += 1;
        }
    }

    println!("{}", ans);
}
