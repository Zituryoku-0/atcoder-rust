use ascii::AsciiChar::X;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
 * dp[i][x]がYesとなるようなdpを作る
 * 階段が無限なので、毎回dp[i][x]がtrueか判定する必要がある
 * Xを超えると、絶対No
 * x<= 10^5なので、行は10^8くらい？
 *
 * Noが確定するタイミング
 * ・dp[i-1][j] で、j > Xの場合
 * ・dp[i-1][j]で、j <= Xのものが1つもない
 *
 */

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    }

    let row = 1000;
    let mut dp = vec![vec![false; x + 1]; row + 1];

    dp[0][0] = true;

    for i in 1..=row {
        let mut count = 0;
        for j in 0..x {
            if dp[i - 1][j] {
                count += 1;
                for k in 0..a.len() {
                    if j + a[k] > x {
                        break;
                    }
                    dp[i][j + a[k]] = true;
                }
            }

            if count < 1 {
                println!("No");
                return;
            }
        }

        // X段目に行けたか
        if dp[i][x] {
            println!("Yes");
            return;
        }

        // もちの考慮
        for j in 0..m {
            dp[i][b[j]] = false;
        }
    }
}
