use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::print;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        s: String,
    }

    let len = s.len() - 1;
    let chrs: Vec<char> = s.chars().collect();
    if chrs[len] == 'e' {
        println!("{}", s + "r");
    } else {
        println!("{}", s + "er");
    }
}
