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
        s: [Chars; n],
    }

    let mut patterns = HashSet::new();

    // M * Mの領域の左上座標を全探索
    for i in 0..=n - m {
        for j in 0..=n - m {
            let mut pattern = Vec::new();

            // (i, j)を左上とするM*Mの領域を取り出す
            for x in i..i + m {
                let mut row = String::new();

                for y in j..j + m {
                    row.push(s[x][y]);
                }
                pattern.push(row);
            }

            // 同じ模様は重複して登録されない
            patterns.insert(pattern);
        }
    }

    println!("{}", patterns.len());
}
