use num::abs;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

// #[path = "../../../lib/lib.rs"]
// mod lib;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut ans: Vec<Vec<usize>> = vec![vec![0usize; w]; h];
    let mv: Vec<(i32, i32)> = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];

    for i in 0..h {
        for j in 0..w {
            for (x, y) in &mv {
                let ni = i as i32 + x;
                let nj = j as i32 + y;
                // 隣接判定
                if ni >= 0 && ni < h as i32 && nj >= 0 && nj < w as i32 {
                    ans[i][j] += 1;
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", ans[i][j]);
            if j < w - 1 {
                print!("{}", ' ');
            }
        }
        if i < h {
            println!();
        }
    }
}
