use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
    }

    // 解けなかった問題
    // 貪欲法で解くのが良い

    let mut items: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            c:usize,
            v:usize,
        }
        let c = c - 1;
        items[c].push(v);
    }

    let mut top: Vec<usize> = Vec::new();
    let mut other: Vec<usize> = Vec::new();
    for i in 0..n {
        if items[i].len() == 0 {
            continue;
        }
        items[i].sort();
        top.push(items[i].pop().unwrap());
        other.append(&mut items[i]);
    }
    top.sort();
    let mut ans = 0;
    for _ in 0..m {
        ans += top.pop().unwrap();
    }

    other.append(&mut top);

    other.sort();

    for _ in 0..k - m {
        ans += other.pop().unwrap();
    }
    println!("{}", ans);
}
