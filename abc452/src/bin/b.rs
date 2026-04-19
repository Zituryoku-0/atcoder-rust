use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut grid = vec![String::new(); h];

    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
