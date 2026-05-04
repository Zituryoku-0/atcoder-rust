use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }
    let mut n = n;

    let base: usize = 10;
    if n <= base.pow(3) - 1 {
        println!("{}", n);
    } else if n <= base.pow(4) - 1 {
        n = n / 10 * 10;
        println!("{}", n);
    } else if n <= base.pow(5) - 1 {
        n = n / 100 * 100;
        println!("{}", n);
    } else if n <= base.pow(6) - 1 {
        n = n / 1000 * 1000;
        println!("{}", n);
    } else if n <= base.pow(7) - 1 {
        n = n / 10000 * 10000;
        println!("{}", n);
    } else if n <= base.pow(8) - 1 {
        n = n / 100000 * 100000;
        println!("{}", n);
    } else if n <= base.pow(9) - 1 {
        n = n / 1000000 * 1000000;
        println!("{}", n);
    }
}
