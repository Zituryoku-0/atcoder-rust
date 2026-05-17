use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::collections::{BTreeMap, HashMap, HashSet};

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n],
    }

    let mut ac = 1;
    let mut wa = 2e18 as usize + 1;

    let judge = |_wj: usize| -> bool {
        let mut rem: i128 = k as i128;
        for i in 0..n {
            if _wj <= a[i] {
                continue;
            }
            rem -= lib::divceil((_wj - a[i]) as i128, (i + 1) as i128);
            if rem < 0 {
                return false;
            }
        }
        true
    };

    // 二分探索する
    while wa - ac > 1 {
        let wj = (ac + wa) / 2;
        if judge(wj) {
            ac = wj;
        } else {
            wa = wj;
        }
    }
    println!("{}", ac);
}
