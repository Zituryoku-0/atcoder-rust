use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;
use std::process::id;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // let mut ans = vec![0; n];
    // println!("m / n：{}", m / n);
    // for i in 0..(m / n) + 1 {
    //     for j in 0..n {
    //         if (i + 1) * (j + 1) <= m {
    //             if i == 12 {
    //                 println!("(i + 1) * (j + 1)：{}", (i + 1) * (j + 1));
    //             }

    //             ans[j] += 1;
    //         }
    //     }
    // }

    for i in 0..n {
        if i < m % n {
            println!("{}", m / n + 1);
        } else {
            println!("{}", m / n);
        }
    }
}
