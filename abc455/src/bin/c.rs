use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let num = 1_000_000_000;
    let mut a_arr: Vec<usize> = vec![0; num];

    for i in 0..n {
        input! {
            a: usize,
        }
        a_arr[a - 1] += a;
    }

    a_arr.sort();

    for i in 0..k {
        a_arr.pop();
    }

    let mut ans = 0;
    for i in 0..a_arr.len() {
        ans += a_arr[i];
    }

    println!("{}", ans);
}
