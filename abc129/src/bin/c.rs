use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
 * 別解
 * 今回は、壊れている階段を別のvecで管理する
 * これだと、アルゴリズムがシンプルになる
 *
 * また、一度にdp[i] = dp[i-1] + dp[i-2]でも良いが
 * dp[i] += dp[i-1]してから、i >= 2の時はdp[i] += dp[i-2]とすると、
 * dp[0]のみ個別管理して、dp[1]からループに組み込むことができるので、こちらの方が実装としては良い
 *
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let num = 1000000007;

    let mut dp: Vec<i128> = vec![0; n + 1];
    let mut broken = vec![false; n + 1];

    // 壊れている階段をfalseにする
    for i in 0..m {
        broken[a[i]] = true;
    }

    // 最初は何もしないという1通りがある
    dp[0] = 1;

    for i in 1..=n {
        // iがaと一致する場合は、0とする
        if broken[i] {
            continue;
        }
        dp[i] += dp[i - 1];

        if i >= 2 {
            dp[i] += dp[i - 2];
        }
        dp[i] %= num;
    }
    // println!("{:?}", dp);

    println!("{}", dp[n]);
}
