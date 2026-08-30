use im_rc::hashmap::Values;
use im_rc::HashMap;
use indexmap::map;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::print;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut map = HashMap::new();

    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 0;

    for (k, v) in map.iter() {
        if v % 2 == 1 {
            ans += k;
        }
    }

    println!("{}", ans);
}
