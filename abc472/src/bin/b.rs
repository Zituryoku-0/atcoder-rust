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
        n: usize,
    }

    let mut total: i64 = 0;
    let mut diff_ary: Vec<i64> = vec![0; n];
    let mut l_ary: Vec<i32> = Vec::new();
    let mut ans = i64::MAX;

    for i in 0..n {
        input! {
            l: i32,
        }

        if i > 0 {
            diff_ary[i] = diff_ary[i - 1] + l as i64;
        } else {
            diff_ary[i] = l as i64;
        }
        total += l as i64;
        l_ary.push(l);
    }

    for i in 0..n {
        let check_num_right = (total - diff_ary[i] as i64).abs();
        let check_num = (check_num_right - diff_ary[i] as i64).abs();
        ans = ans.min(check_num);
    }
    println!("{}", ans);
}
