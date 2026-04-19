use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![0usize; n];
    let mut b = vec![0usize; n];

    for i in 0..n {
        input! {
            ai: usize,
            bi: usize,
        }
        a[i] = ai;
        b[i] = bi;
    }

    input! {
        m: usize,
        s: [String; m],
    }

    // can[len][pos][ch] = 長さlenの文字列で、pos文字目がchであるものが存在するか
    // len, posは1-indexed
    let mut can = vec![vec![vec![false; 26]; 11]; 11];

    for word in &s {
        let chars: Vec<char> = word.chars().collect();
        let len = chars.len();

        for pos in 0..len {
            let ch_idx = (chars[pos] as u8 - b'a') as usize;
            can[len][pos + 1][ch_idx] = true;
        }
    }

    for word in &s {
        let chars: Vec<char> = word.chars().collect();

        // 脊椎の長さはN出なければならない
        if chars.len() != n {
            println!("No");
            continue;
        }
        let mut ok = true;

        for i in 0..n {
            let ch_idx = (chars[i] as u8 - b'a') as usize;

            // 肋骨iに対して
            // 長さ A_iの文字列で、B_i文字目がchars[i]のものが存在するか
            if !can[a[i]][b[i]][ch_idx] {
                ok = false;
                break;
            }
        }
        println!("{}", if ok { "Yes" } else { "No" });
    }
}
