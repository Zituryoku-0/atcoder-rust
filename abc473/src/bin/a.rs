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
        a: [i32; n],
    }

    let mut ans = 0;
    for i in n / 2..n {
        ans += a[i];
    }

    println!("{}", ans);
}
