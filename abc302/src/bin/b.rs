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
    let h = h - 1;
    let w = w - 1;

    let dirs = [
        (1 as i32, 0 as i32),
        (1, -1),
        (0, -1),
        (-1 as i32, -1 as i32),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let snuck: Vec<char> = "snuke".chars().collect();

    for i in 0..h + 1 {
        for j in 0..w + 1 {
            for (k, l) in dirs {
                let mut ans = Vec::new();
                let mut ok = true;
                for m in 0..5 {
                    let dx = (i as i32 + k * m) as i32;
                    let dy = (j as i32 + l * m) as i32;
                    if dx < 0 || dx > h as i32 || dy < 0 || dy > w as i32 {
                        ok = false;
                        break;
                    }

                    if s[dx as usize][dy as usize] != snuck[m as usize] {
                        ok = false;
                        break;
                    }
                    ans.push((dx, dy));
                }

                if ok {
                    for (r, c) in ans {
                        println!("{} {}", r + 1, c + 1);
                    }
                }
            }
        }
    }
}
