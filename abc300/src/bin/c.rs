use either::Either::Left;
use proconio::{input, marker::Chars};
use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h], // これは実質Vec<Vec<char>>である
    }
    let n = min(h, w);
    let mut ans = vec![0usize; n];

    for i in 0..h {
        for j in 0..w {
            // 中心は#でないといけない
            if grid[i][j] != '#' {
                continue;
            }

            let mut size = 0usize;
            let mut d = 1usize;

            loop {
                // 4方向の斜め先
                let up = i.checked_sub(d);
                let left = j.checked_sub(d);
                let down = i + d;
                let right = j + d;

                // 範囲外なら終了
                if up.is_none() || left.is_none() || down >= h || right >= w {
                    break;
                }

                let ui = up.unwrap();
                let lj = left.unwrap();

                if grid[ui][lj] == '#'
                    && grid[ui][right] == '#'
                    && grid[down][lj] == '#'
                    && grid[down][right] == '#'
                {
                    size += 1;
                    d += 1;
                } else {
                    break;
                }
            }
            if size > 0 {
                ans[size - 1] += 1;
            }
        }
    }

    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans[i]);
    }
    println!();
}
