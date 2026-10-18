use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        grid: [Chars; 2], // これは実質Vec<Vec<char>>である
    }

    let direction: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    for i in 0..2 {
        for j in 0..2 {
            if grid[i][j] == 'A' {
                for (x, y) in direction {
                    let dx = i as i32 + x;
                    let dy = j as i32 + y;
                    // マスを超えたらスキップ
                    if dx < 0 || dy < 0 || dx >= 2 || dy >= 2 {
                        continue;
                    }
                    if grid[dx as usize][dy as usize] == 'D' {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
