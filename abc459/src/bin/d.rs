use im_rc::HashMap;
use num_integer::div_ceil;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec;

#[path = "../../../lib/lib.rs"]
mod lib;

// 切り上げ割り算をする
pub fn divceil(a: i128, b: i128) -> i128 {
    (a + b - 1) / b
}

fn main() {
    input! {
        t: usize,
        cases: [String; t], // これは実質Vec<Vec<char>>である
    }

    let mut check: i128 = 0;
    for i in 0..t {
        // 各文字列の個数をカウントする
        let mut cnt: Vec<usize> = vec![0usize; 26];
        for c in cases[i].chars() {
            let idx = (c as u8 - b'a') as usize;
            cnt[idx] += 1;
        }
        cnt.sort();

        if div_ceil(cases[i].len(), cnt[25]) > 1 {
            println!("No");
            break;
        }

        let mut ans: Vec<char> = vec![' '; cases[i].len()];

        for k in (0..cases[i].len()).step_by(2) {
            ans[k] = cnt[]
        }
    }
}
