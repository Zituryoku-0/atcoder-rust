use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::print;
use std::println;
use std::vec;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
    }

    let mut a_ary = Vec::with_capacity(n);
    let mut a_ary_sort = Vec::with_capacity(n);

    for i in 0..n {
        input! {
            a: i64,
        }
        a_ary.push(a);
    }
    a_ary_sort = a_ary.clone();
    a_ary_sort.sort();

    let mut ans = vec![0i64; n];

    let mut total = 0;

    for i in (0..n - 1).rev() {
        total += a_ary_sort[i + 1];
        if a_ary_sort[i + 1] > a_ary_sort[i] {
            ans[i] = total;
        } else {
            ans[i] = ans[i + 1];
        }
    }

    for i in 0..n {
        let idx = a_ary_sort.partition_point(|&v| v < a_ary[i]);
        print!("{}", ans[idx]);
        if i < n {
            print!(" ");
        }
    }
    println!();
}
