use proconio::{input, marker::Chars};
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

// #[path = "../../../lib/lib.rs"]
// mod lib;

fn main() {
    input! {
        s:String,
    }
    let mut ans: i64 = 0;
    let len: usize = s.len();
    let mut chars: Vec<char> = s.chars().collect();

    for i in 0..len {
        let mut to_zero: i64 = 0;
        let mut to_end: i64 = 0;
        let mut min_len = 0;
        if chars[i] == 'C' {
            to_zero = i as i64 - 0;
            to_end = (len - 1) as i64 - i as i64;
            min_len = to_zero.min(to_end);
            // println!("min_lenの値：{}", min_len);
            ans += min_len + 1;
        }
    }
    println!("{}", ans);
}
