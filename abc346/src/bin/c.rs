use ascii::AsciiChar::Hash;
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
 * 解説
 * 1からkまでの総和は、k * (k + 1) / 2でも止めることができる
 * hashsetのinsertの返り値はboolなので、a[i] <= k && set.insert(&a[i])の条件を満たすもののみ
 * a_totalに入れれば良い
 * 最後にk_total - a_totalとすれば良い
 */

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut set = HashSet::new();

    let k_total = k * (k + 1) / 2;

    let mut a_total = 0;
    for i in 0..n {
        if a[i] <= k && set.insert(&a[i]) {
            a_total += a[i];
        }
    }

    println!("{}", k_total - a_total);
}
