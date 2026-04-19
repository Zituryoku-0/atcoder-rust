use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let mut count_s_chars: Vec<usize> = vec![0usize; 26];
    let mut count_t_chars: Vec<usize> = vec![0usize; 26];
    let mut s_chars: Vec<char> = s.chars().collect();
    let at_chars: Vec<char> = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    let mut s_at_count: i32 = 0;
    let mut t_at_count: i32 = 0;
    let mut t_chars: Vec<char> = t.chars().collect();

    for i in 0..s_chars.len() {
        if s_chars[i] == '@' {
            s_at_count += 1;
            continue;
        }
        let current_char = (s_chars[i] as u8 - b'a') as usize;
        count_s_chars[current_char] += 1;
    }

    for i in 0..t_chars.len() {
        if t_chars[i] == '@' {
            t_at_count += 1;
            continue;
        }
        let current_char = (t_chars[i] as u8 - b'a') as usize;
        count_t_chars[current_char] += 1;
    }
    for i in 0..26 {
        if count_s_chars[i] == count_t_chars[i] {
            continue;
        }

        let ch = (b'a' + i as u8) as char;

        // この文字は@で補えるか
        let can_replace = at_chars.contains(&ch);

        if !can_replace {
            println!("No");
            return;
        }

        if count_s_chars[i] > count_t_chars[i] {
            // T側が不足しているので、Tの@で補う
            let diff = count_s_chars[i] - count_t_chars[i];
            t_at_count -= diff as i32;
            if t_at_count < 0 {
                println!("No");
                return;
            }
        } else {
            // S側が不足しているので、Sの@で補う
            let diff = count_t_chars[i] - count_s_chars[i];
            s_at_count -= diff as i32;
            if s_at_count < 0 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
