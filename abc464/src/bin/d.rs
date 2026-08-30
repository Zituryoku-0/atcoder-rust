use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: String,
            x: [i64; n],
            y: [i64; n-1],
        }

        let s = s.as_bytes();

        // dp_s:
        // 現在の日が晴れ（S）の場合の最大嬉しさ
        //
        // dp_r:
        // 現在の日が雨（R）の場合の最大嬉しさ
        let (mut dp_s, mut dp_r);

        if s[0] == b'S' {
            dp_s = 0;
            dp_r = -x[0];
        } else {
            dp_r = 0;
            dp_s = -x[0];
        }

        for i in 1..n {
            // 今日を晴れにする場合
            //
            // S -> S：ボーナスなし
            // R -> S：y[i-1]のボーナス
            let mut next_s = dp_s.max(dp_r + y[i - 1]);

            // 今日を雨にする場合
            //
            // S -> R, R -> R
            // どちらもボーナスなし
            let mut next_r = dp_s.max(dp_r);

            // 元の天気から変更した場合はコストを払う
            if s[i] == b'S' {
                next_r -= x[i];
            } else {
                next_s -= x[i];
            }

            dp_s = next_s;
            dp_r = next_r;
        }
        println!("{}", dp_s.max(dp_r));
    }
}
