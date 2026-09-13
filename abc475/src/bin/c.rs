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
 * 訪れる街の集合はSを含む区間になる
 * すなわち、l <= S <= rを満たす整数l, rが存在し、訪れる街の集合はl以上r以下の街の集合となる
 * l,rを決め打致死、その時の移動距離として考えられる最小値を求める
 * この最小値がL以下となるl, rについて、r-l+1の最大値を求めれば良い
 *
 * 街lと街Sの距離をX、街Sと街rの距離をYとする
 * 街lを訪れた後に街rを訪れる時の移動距離の最小値は2X + Y、街rを訪れた後に街lを訪れる時の移動距離の最小は2Y + Xのため
 * 最小値は(2x + Y).min(x + 2Y)である
 *
 *
 */

fn main() {
    input! {
        n: usize,
        s: usize,
        l: usize,
        a: [usize; n-1],
    }

    let s = s - 1;

    let mut ans = 1;
    let mut p = vec![0; n];
    // 距離の累積和
    for i in 0..n - 1 {
        p[i + 1] = p[i] + a[i];
    }

    for left in 0..=s {
        for right in s..n {
            let x = p[s] - p[left];
            let y = p[right] - p[s];
            if (2 * x + y).min(x + 2 * y) <= l {
                ans = ans.max(right - left + 1);
            }
        }
    }
    println!("{}", ans);
}
