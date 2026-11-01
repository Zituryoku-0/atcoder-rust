use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::print;
use std::println;

fn main() {
    input! {
        s: String,
    }

    let digit = s.len();

    let mut del = digit / 2;

    let mut chrs: Vec<char> = s.chars().collect();

    for i in 0..digit {
        if i == del {
            continue;
        }
        print!("{}", chrs[i]);
    }
}
