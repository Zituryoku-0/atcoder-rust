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
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut ans = vec![0usize; n];

    for i in 0..m {
        let mut diff_count = vec![0; n];
        let mut check = false;
        for j in 0..n {
            if t[i] == s[j] {
                ans[j] += 1;
                check = true;
                break;
            }
            let mut t_chrs: Vec<char> = t[i].chars().collect();
            let mut s_chrs: Vec<char> = s[j].chars().collect();
            // 文字数が一致しないならスキップ
            if t_chrs.len() != s_chrs.len() {
                continue;
            }

            for k in 0..t_chrs.len() {
                if t_chrs[k] != s_chrs[k] {
                    diff_count[j] += 1;
                }
            }
        }
        if !check {
            let mut diff = Vec::new();
            for j in 0..n {
                if diff_count[j] == 1 {
                    diff.push(j);
                }
            }

            if diff.len() == 1 {
                ans[diff[0]] += 1;
            }
        }
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}
