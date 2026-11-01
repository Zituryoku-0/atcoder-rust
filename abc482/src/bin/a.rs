use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        s: String,
    }

    let mut chrs: Vec<char> = s.chars().collect();

    let mut ans = 0;
    for i in 0..chrs.len() {
        if chrs[i] == 'o' {
            ans += 1;
        }
    }

    if ans % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }
}
