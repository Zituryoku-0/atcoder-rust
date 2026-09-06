use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/***
 * p[i] % 10して、前 > 後が一度でもあればNo
 *
 *
 *
 */
fn main() {
    input! {
        n: usize,
        mut p: [i32;n],
    }

    for i in 0..n {
        if p[i] % 10 == 0 {
            p[i] -= 1;
        }
        p[i] = p[i] / 10;
    }

    // println!("{:?}", p);

    for i in 1..n {
        if p[i - 1] > p[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
