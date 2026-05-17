use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    // let k = k - 1;
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
        c:[usize; n],
    }

    let mut counter = 0;
    let mut diff = 0;
    for i in 0..n {
        if k <= counter + (a[i].len() * c[i]) {
            diff = k - counter;
            if diff % a[i].len() == 0 {
                println!("{}", a[i][a[i].len() - 1]);
                return;
            } else {
                println!("{}", a[i][diff % a[i].len() - 1]);
                return;
            }
        }
        counter += a[i].len() * c[i];
    }
}
