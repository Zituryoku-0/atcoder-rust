use im_rc::HashMap;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::new();

    for name in s {
        *map.entry(name).or_insert(0) += 1;
    }

    let mut ans = String::new();
    let mut max_num = 0;

    for (name, count) in map {
        let mut temp = max_num;

        max_num = max_num.max(count);

        if temp != max_num {
            ans = name;
        }
    }

    println!("{}", ans);
}
