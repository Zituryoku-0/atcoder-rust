use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
 * *以外の文字が一致すればOK
 *
 */

fn main() {
    input! {
        n: usize,
        s: String,
        t: String,
    }

    let s_chrs: Vec<char> = s.chars().collect();
    let t_chrs: Vec<char> = t.chars().collect();

    for i in 0..n {
        if t_chrs[i] == '*' {
            continue;
        }
        if s_chrs[i] != t_chrs[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
