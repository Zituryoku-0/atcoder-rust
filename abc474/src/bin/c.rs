use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::print;

#[path = "../../../lib/lib.rs"]
mod lib;

/***
 *
 * a[i]を逆から判定すれば良い
 *
 *
 *
 *
 */
fn main() {
    input! {
        n: usize,
        q: usize,
        p: [usize; n],
        a: [usize; q],
    }

    let mut is_del = vec![false; n];
    let mut back = Vec::new();
    for i in (0..q).rev() {
        if !is_del[a[i] - 1] {
            is_del[a[i] - 1] = true;
            back.push(a[i]);
        }
    }

    for i in 0..n {
        if is_del[p[i] - 1] {
            continue;
        }
        print!("{}", p[i]);
        print!("{}", ' ');
    }

    for i in (0..back.len()).rev() {
        print!("{}", back[i]);
        print!("{}", ' ');
    }
}
