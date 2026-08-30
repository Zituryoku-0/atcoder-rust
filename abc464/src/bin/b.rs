use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::print;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars; h], // これは実質Vec<Vec<char>>である
    }

    // 最上行
    let mut check = true;
    while check {
        for j in 0..w {
            if c[0][j] != '.' {
                check = false;
                break;
            }
        }
        if !check {
            break;
        }
        c.remove(0);
    }

    let mut check = true;
    // 最下行
    while check {
        let len = c.len();
        for j in 0..w {
            if c[len - 1][j] != '.' {
                check = false;
                break;
            }
        }
        if !check {
            break;
        }
        c.remove(len - 1);
    }

    let h_len = c.len();
    let mut check = true;

    // 最左行
    while check {
        for i in 0..h_len {
            if c[i][0] != '.' {
                check = false;
                break;
            }
        }
        if !check {
            break;
        }
        // 各行の0番目を削除
        for i in 0..h_len {
            c[i].remove(0);
        }
    }

    let mut check = true;

    // 最右行
    while check {
        let w_len = c[0].len();

        for i in 0..h_len {
            if c[i][w_len - 1] != '.' {
                check = false;
                break;
            }
        }
        if !check {
            break;
        }
        // 各行の0番目を削除
        for i in 0..h_len {
            c[i].remove(w_len - 1);
        }
    }

    let w_len = c[0].len();

    for i in 0..h_len {
        for j in 0..w_len {
            print!("{}", c[i][j]);
        }
        if i < h_len {
            println!();
        }
    }
}
