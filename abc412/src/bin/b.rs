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
        t: String,
    }

    let mut s_chrs: Vec<char> = s.chars().collect();
    let mut t_chrs: Vec<char> = t.chars().collect();

    // is_uppercase()
    for i in 1..s_chrs.len() {
        let mut check = false;
        if !s_chrs[i].is_uppercase() {
            continue;
        }

        for j in 0..t_chrs.len() {
            if s_chrs[i - 1] == t_chrs[j] {
                check = true;
                break;
            }
        }
        if !check {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
