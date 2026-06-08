use ascii::AsciiChar::N;
use ascii::AsciiChar::X;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

// #[path = "../../../lib/lib.rs"]
// mod lib;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut querys: Vec<(usize, usize)> = Vec::new();
    for _ in 0..q {
        input! {
            query: (usize, usize),
        }
        querys.push(query);
    }

    let mut h: Vec<usize> = vec![0usize; n];

    let mut c: Vec<usize> = vec![0usize; 1e6 as usize];

    let mut l = 0;

    for i in 0..q {
        let q_type = querys[i].0;
        if q_type == 1 {
            let x = querys[i].1 - 1;
            h[x] += 1;
            c[h[x]] += 1;

            if c[h[x]] == n {
                l += 1;
            }
        } else {
            println!("{}", c[querys[i].1 + l]);
        }
    }
}
