use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

fn main() {
    input! {
        n: usize,
        a: [i128; n],
    }

    let mut a_ary = a.clone();
    a_ary.sort();

    let mut ans = vec![0i128; n];

    for i in 0..n {
        if a[i] == a_ary[n - 1] {
            ans[i] = a_ary[n - 2];
        } else {
            ans[i] = a_ary[n - 1];
        }
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}
