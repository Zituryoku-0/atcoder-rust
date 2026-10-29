use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        q: usize,
    }

    let mut count = 0;
    let mut map: HashMap<String, usize> = HashMap::new();
    for _ in 0..q {
        input! {
            s: String,
        }

        match map.get(&s) {
            Some(idx) => println!("{}", idx),
            None => {
                count += 1;
                map.insert(s, count);
                println!("{}", count);
            }
        }
    }
}
