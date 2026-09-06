use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

fn dfs(mut ans: i128, a_ary: &[Vec<i128>], x: i128, n: usize, current: usize, sum: i128) -> i128 {
    // println!("currentの確認：{}", current);
    // println!("sumの確認：{}", sum);
    // ボールを最後まで選択したか
    if current >= n {
        // Xと一致したか
        if sum == x {
            // println!("sum == xだよ");
            ans += 1;
        }
        return ans;
    }

    if sum > x {
        return ans;
    }

    // ボールを選択するパターン
    for i in 0..a_ary[current].len() {
        // println!("a_ary[current][i]の確認：{}", a_ary[current][i]);
        ans = dfs(ans, a_ary, x, n, current + 1, sum * a_ary[current][i]);
    }

    ans
}

fn main() {
    input! {
        n: usize,
        x: i128,
    }

    let mut a_ary: Vec<Vec<i128>> = vec![Vec::new(); n];

    for i in 0..n {
        input! {
            l: usize,
            a: [i128; l],
        }
        a_ary[i] = a;
    }

    let mut current = 0;
    let mut sum = 1;
    let ans = dfs(0, &a_ary, x, n, current, sum);

    println!("{}", ans);
}
