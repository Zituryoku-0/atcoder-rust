use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

const DI: [isize; 4] = [-1, 0, 1, 0];
const DJ: [isize; 4] = [0, -1, 0, 1];

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut queue = VecDeque::new();
    let mut inf = i32::MAX;
    let mut dist = vec![vec![inf; w]; h];

    // 黒色マスの特定（distが0）
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                dist[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }

    // 多始点BFS
    while let Some((i, j)) = queue.pop_front() {
        // 上下左右を黒色にする
        // 行の色変え
        for v in 0..4 {
            let mut ni = i as isize + DI[v];
            let mut nj = j as isize + DJ[v];

            // グリッドの範囲外ならスキップ
            if ni < 0 || nj < 0 || ni >= h as isize || nj >= w as isize {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            // すでにdistが更新されていればスキップ
            if dist[ni][nj] != inf {
                continue;
            }

            dist[ni][nj] = dist[i][j] + 1;
            queue.push_back((ni, nj));
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(dist[i][j]);
        }
    }

    println!("{}", ans);
}
