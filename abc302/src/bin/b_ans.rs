use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let word: Vec<char> = "snuke".chars().collect();

    // 8方向
    let dirs = vec![
        (-1_i32, -1_i32),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..h {
        for j in 0..w {
            for &(dx, dy) in &dirs {
                let mut positions = Vec::new();
                let mut ok = true;

                for k in 0..5 {
                    let ni = i as i32 + dx * k as i32;
                    let nj = j as i32 + dy * k as i32;

                    // 範囲外なら失敗
                    if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
                        ok = false;
                        break;
                    }

                    if s[ni as usize][nj as usize] != word[k] {
                        ok = false;
                        break;
                    }

                    positions.push((ni as usize + 1, nj as usize + 1)); // 1 - indexed
                }
                if ok {
                    for (r, c) in positions {
                        println!("{} {}", r, c);
                    }
                    return;
                }
            }
        }
    }
}
