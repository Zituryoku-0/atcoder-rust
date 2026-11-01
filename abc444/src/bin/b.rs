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
        k: usize,
    }

    let mut ans = 0;
    for i in 1..=n {
        let mut x = i;
        let mut temp = 0;
        while x > 0 {
            temp += x % 10;
            x /= 10;
        }

        if temp == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
