use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        s: String,
    }

    const MOD: i64 = 998244353;

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    let mut ans: i64 = 0;

    // 現在位置を右端としたとき
    // 条件を満たす部分文字列の個数
    let mut len: i64 = 0;

    for i in 0..n {
        if i == 0 {
            len = 1;
        } else {
            if chars[i - 1] != chars[i] {
                len += 1;
            } else {
                len = 1;
            }
        }
        ans += len;
        ans %= MOD;
    }
    println!("{}", ans);
}
