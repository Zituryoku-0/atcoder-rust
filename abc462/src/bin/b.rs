use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
    }

    let mut gifts: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut counts: Vec<usize> = vec![0usize; n];

    for i in 0..n {
        input! {
            k: usize,
        }
        for j in 0..k {
            input! {
                a: usize,
            }
            let a = a - 1;
            counts[a] += 1;
            gifts[a].push(i);
        }
    }

    for i in 0..n {
        print!("{} ", counts[i]);
        if counts[i] == 0 {
            println!();
        } else {
            for j in 0..counts[i] {
                print!("{}", gifts[i][j] + 1);
                if j < counts[i] {
                    print!(" ");
                }
            }
        }
        println!();
    }
}
