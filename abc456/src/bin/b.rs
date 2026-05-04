use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let mut a: Vec<Vec<usize>> = vec![vec![0; 6]; 3];

    let waru = 216;

    for i in 0..3 {
        for j in 0..6 {
            input! {
                v: usize,
            }
            a[i][j] = v;
        }
    }

    let mut count = 0;
    for i in 0..6 {
        if a[0][i] == 4 || a[0][i] == 5 || a[0][i] == 6 {
            for j in 0..6 {
                if (a[1][j] == 4 || a[1][j] == 5 || a[1][j] == 6) && a[0][i] != a[1][j] {
                    for k in 0..6 {
                        if (a[2][k] == 4 || a[2][k] == 5 || a[2][k] == 6)
                            && a[0][i] != a[2][k]
                            && a[1][j] != a[2][k]
                        {
                            count += 1;
                            // println!("countがインクリメント時の各数字：{} {} {}", i, j, k);
                        } else {
                            continue;
                        }
                    }
                } else {
                    continue;
                }
            }
        } else {
            continue;
        }
    }
    // println!("countの個数：{}", count);

    println!("{}", count as f32 / waru as f32 as f32);
}
