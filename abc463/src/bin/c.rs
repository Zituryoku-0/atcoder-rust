use ascii::AsciiChar::Q;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;
use std::process::id;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        hl: [(u64, u64); n],
        q: usize,
        t: [u64; q],
    }

    let mut h = Vec::with_capacity(n);
    let mut l = Vec::with_capacity(n);

    for &(hi, li) in &hl {
        h.push(hi);
        l.push(li);
    }

    // suffix[i] = i番目以降の最大身長
    let mut suffix = vec![0u64; n];

    suffix[n - 1] = h[n - 1];

    for i in (0..n - 1).rev() {
        suffix[i] = suffix[i + 1].max(h[i]);
    }

    for ti in t {
        // 最初にL_i > Tとなる１
        let idx = l.partition_point(|&li| li <= ti);

        println!("{}", suffix[idx]);
    }
}
