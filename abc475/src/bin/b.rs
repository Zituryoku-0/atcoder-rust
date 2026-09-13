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
        n: usize,
        a: [usize; n],
    }
    // 使った金額に対して最低限の1000札を出せば良い
    let mut one = 0;
    let mut ten = 0;
    let mut handred = 0;

    for i in 0..n {
        let mut current = a[i];
        // まず、1000円札が何枚必要か
        let count = (a[i] / 1000) + 1;

        let mut diff = count * 1000 - current;
        one += diff % 10;
        diff = diff / 10;
        ten += diff % 10;
        diff = diff / 10;
        handred += diff % 10;
    }

    print!("{}", one);
    print!("{}", ' ');
    print!("{}", ten);
    print!("{}", ' ');
    print!("{}", handred);
    println!();
}
