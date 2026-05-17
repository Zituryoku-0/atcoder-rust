use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }
    let n = n;
    let mut a = vec![vec![]; n];
    for i in 0..n {
        input! {
            l: usize,
        }
        for j in 0..l {
            input! {
                a_a: usize,
            }
            a[i].push(a_a);
        }
    }
    input! {
        x: usize,
        y: usize,
    }
    let x = x - 1;
    let y = y - 1;

    println!("{}", a[x][y]);
}
