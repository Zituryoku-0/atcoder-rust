use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h],
    }

    for i in 0..h {
        for j in 0..w {
            let mut ok = true;

            for s in 0..h {
                for t in 0..w {
                    if a[(i + s) % h][(j + t) % w] != b[s][t] {
                        ok = false;
                    }
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
