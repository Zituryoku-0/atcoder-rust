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
        s: String,
    }

    let mut ans = 0;
    let chrs: Vec<char> = s.chars().collect();

    let mut counter = 0;
    for i in 0..n {
        if chrs[i] == 'R' {
            counter += 1;
            ans = ans.max(counter);
        } else {
            ans = ans.max(counter);
            counter = 0;
        }
    }

    println!("{}", ans);
}
