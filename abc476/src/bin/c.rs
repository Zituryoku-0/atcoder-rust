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
 * 単純にソートすればいい？
 * N-2行出力するので、ソートと出力が重なるとTLEする
 * ヒープキューに詰め込めばいける
 *
 *
 *
 */

fn main() {
    input! {
        n: usize,

    }

    let mut heap = BinaryHeap::new();

    for i in 0..n {
        input! {
            a: usize,
        }

        heap.push(a);
        if i > 1 {
            let add_a: usize = heap.pop().unwrap();
            let add_b: usize = heap.pop().unwrap();
            match heap.peek() {
                Some(v) => println!("{}", v),
                _ => return,
            }
            heap.push(add_a);
            heap.push(add_b);
        }
    }
}
