use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }

    // 事前にソートしておく
    a.sort();
    let len = a.len();

    for i in 0..q {
        input! {
            x: usize,
        }

        let over_x = a.partition_point(|&v| v < x);
        println!("{}", len - over_x);
    }
}
