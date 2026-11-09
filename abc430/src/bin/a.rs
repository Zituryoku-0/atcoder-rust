use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;
use std::sync::mpsc::RecvTimeoutError;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if c < a {
        println!("No");
        return;
    }

    if c >= a && d >= b {
        println!("No");
        return;
    }

    println!("Yes");
}
