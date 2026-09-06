use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::print;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
 *
 * A[i] > B[i]が1つでもあればYesになりやすい
 * 1つもなければNoは確定する
 *
 * Yesのパターンをどのように出力するか
 * A[i] < B[i]は1で固定する
 * A[i] > B[i]の時は、10^9で出してあげる
 *
 *
 */
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    for i in 0..n {
        if a[i] > b[i] {
            println!("Yes");
            for i in 0..n {
                if a[i] > b[i] {
                    print!("{}", 1000000000000000000i64);
                } else {
                    print!("{}", 1);
                }
                if i < n {
                    print!("{}", ' ');
                }
            }
            println!();
            return;
        }
    }

    println!("No");
}
