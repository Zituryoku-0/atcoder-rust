use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;
use std::thread::current;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        m: i128,
        k: i128,
        a: [i128; n],
    }

    let m = m - 1;

    let mut karory: i128 = 0;
    let mut check: Vec<bool> = vec![false; n];
    for i in 0..n {
        // println!("{}日目、現在カロリー：{}", i, karory);
        // println!("{}日目、追加カロリー：{}", i, a[i]);
        if k >= karory + a[i] {
            karory += a[i];
            println!("Yes");
            check[i] = true;
        } else {
            println!("No");
        }
        if i as i128 - m >= 0 && check[i - m as usize] {
            karory -= a[i - m as usize];
        }
    }
}
