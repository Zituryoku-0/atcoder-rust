use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::print;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;
/***
 * i番目の要素が1 <= i <= Ri
 * N = 3のとき、i1 + i2 + i3 % K == 0であれば良い
 *
 * 全探索で良さそう（o(r^nくらい？)）
 * 辞書順だし、DFSで解いてみる
 *
 * 必要な情報
 * ・current：今どこまで見ているか（nまで行けば終了）
 * ・sum：数列の合計がKの倍数であるかの判定
 *
 */

fn dfs(current: usize, n: usize, k: usize, sum: usize, r: &[usize], ans: &mut Vec<usize>) {
    if current == n {
        for i in 1..=5 {
            if (sum + (i)) % k == 0 && i <= r[current] {
                ans.push(i);
            }
            if ans.len() >= n {
                for j in 0..n {
                    print!("{}", ans[j]);
                    if j + 1 < n {
                        print!("{}", ' ');
                    }
                }
                println!();
                ans.pop();
            }
        }
        return;
    }

    for i in 1..=5 {
        if i <= r[current] {
            ans.push(i);
            dfs(current + 1, n, k, sum + i, r, ans);
            ans.pop();
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }

    let mut sum = 0;

    let mut ans = Vec::new();

    dfs(0, n, k, sum, &r, &mut ans);
}
