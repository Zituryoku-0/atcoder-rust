use ascii::AsciiChar::J;
use num_traits::NumAssign;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
 * 多始点BFSで解くことができる
 *
 * キューを用いて、始点を最初に追加し、その周辺のマスをキューに追加していく
 * これをすると、必然的に始点から距離が近いマスとしてキューに追加されていくため、不要な計算を省くことができる
 *
 */

const DI: [isize; 4] = [-1, 0, 1, 0];
const DJ: [isize; 4] = [0, -1, 0, 1];

fn main() {
    input! {
        h: usize,
        w: usize,
        k: i32,
        s: [Chars; h], // これは実質Vec<Vec<char>>である
    }

    // まずは安全なマスを列挙する
    let mut row = vec![false; h];
    let mut col = vec![false; w];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                row[i] = true;
                col[j] = true;
            }
        }
    }

    let INF = 1001001001;

    let mut dist = vec![vec![INF; w]; h];
    let mut queue = VecDeque::new();

    // '#'が存在しない行かつ'#'が存在しない列のマスを
    // BFSの開始地点とする
    for i in 0..h {
        for j in 0..w {
            if !row[i] && !col[j] {
                dist[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }

    // 多始点BFS
    while let Some((i, j)) = queue.pop_front() {
        for v in 0..4 {
            let ni = i as isize + DI[v];
            let nj = j as isize + DJ[v];

            // 移動した後が、グリッドから出ているかのチェック
            if ni < 0 || nj < 0 || ni >= h as isize || nj >= w as isize {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if s[ni][nj] == '#' {
                continue;
            }

            // すでに訪問済
            if dist[ni][nj] != INF {
                continue;
            }

            dist[ni][nj] = dist[i][j] + 1;
            queue.push_back((ni, nj));
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if dist[i][j] <= k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
