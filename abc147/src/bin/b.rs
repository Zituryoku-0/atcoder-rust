use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        s: String,
    }

    let mut chrs: Vec<char> = s.chars().collect();

    let mut ans = 0;
    let len = chrs.len();
    let mut check = false;

    let len_harf = len / 2;

    // 偶数であれば、全部チェックできる
    if len % 2 == 0 {
        check = true;
    }

    for i in 0..len_harf {
        if i == len_harf && !check {
            break;
        }

        if chrs[i] != chrs[len - 1 - i] {
            // println!("文字が異なる時：{}", chrs[i]);

            ans += 1;
        }
    }

    println!("{}", ans);
}
