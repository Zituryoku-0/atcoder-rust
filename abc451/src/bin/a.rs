use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        s: String,
    }

    if s.len() % 5 == 0 {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
