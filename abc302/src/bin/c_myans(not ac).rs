use ascii::ToAsciiChar;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::usize;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut u_words: Vec<Vec<usize>> = vec![vec![0usize; m]; n];
    let mut total: Vec<Vec<usize>> = vec![vec![0usize; m]; n];

    for i in 0..n {
        for j in 0..m {
            u_words[i][j] = (s[i][j] as u8 - b'a') as usize;
            if j == 0 {
                total[i][j] += u_words[i][j];
            } else {
                total[i][j] += total[i][j - 1] + u_words[i][j];
            }
        }
    }

    // 並べ替え
    for j in 0..m - 1 {
        for i in 0..n - 1 {
            if u_words[i][j] > u_words[i + 1][j] && total[i][j] >= total[i + 1][j] {
                u_words.swap(i, i + 1);
            }
        }
    }

    for i in 0..n - 1 {
        let mut change = 0;
        for j in 0..m {
            if u_words[i][j] != u_words[i + 1][j] {
                change += 1;
            }
        }
        if change != 1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
