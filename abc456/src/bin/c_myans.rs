use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec;

fn main() {
    input! {
        s: String,
    }

    let s_chars: Vec<char> = s.chars().collect();
    let s_len = s_chars.len();
    let waru = 998244353;

    let mut ans: i64 = 0;
    let mut i = 0;

    while i < s_len {
        // println!();
        // println!("iの数：{}", i);
        let mut temp = s_chars[i];
        let mut count = 0;
        for j in i + 1..s_len {
            let char_1 = temp;
            let char_2 = s_chars[j];

            // println!("char1: {}, char2: {}", char_1, char_2);

            if char_1 != char_2 {
                temp = char_2;
                count += 1;
            }

            if char_1 == char_2 || j == s_len - 1 {
                let add_num: i64 = (0..=count).sum();
                // println!("add_numの中身：{}", add_num);

                ans += add_num;
                i = j - 1;
                break;
            }
        }
        // println!("ansの中身：{}", ans);
        i += 1;
    }
    println!("{}", (ans + s_len as i64) % waru);
}
