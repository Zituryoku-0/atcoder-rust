use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
* 【問題の目的】
* ・最終的に何を求める問題か
・H, Wまで行くのに、snukeという文字列が存在するかを求める問題
*
* 【制約から分かること】
* ・N <= 25000
* ・O(N^2) は可能
*
* 【Observation（問題から分かった事実）】
・同じマスでは、その先に進める候補も同じ
・そのため、一度訪問したますは再び探索しなくて良い
*
* 【状態・操作の整理】
* ・何が変化するか：
* ・何が変化しないか：
* ・答えを決めるために必要な情報：
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
*
* 【解法】
* ・上記まで整理すると、〜で解ける
*
* 【計算量】
* ・O(HW)(500 * 500 <= 250000)
*/

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut visited = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();

    if s[0][0] != 's' {
        println!("No");
        return;
    }
    // 最初の要素は確認済みにする
    visited[0][0] = true;
    queue.push_back((0usize, 0usize));

    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    // キューが存在するまで続ける
    while let Some((row, col)) = queue.pop_front() {
        let next_chr = match s[row][col] {
            's' => 'n',
            'n' => 'u',
            'u' => 'k',
            'k' => 'e',
            'e' => 's',
            _ => continue,
        };

        for &(dr, dc) in &directions {
            let nr = row as isize + dr;
            let nc = col as isize + dc;
            // グリッドの範囲外に行こうとしたら終わり
            if nr < 0 || nc < 0 || nr >= h as isize || nc >= w as isize {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;

            // すでに訪れていればスキップ
            if visited[nr][nc] {
                continue;
            }

            // 次の文字がmatchと異なる場合はスキップ
            if s[nr][nc] != next_chr {
                continue;
            }

            visited[nr][nc] = true;
            queue.push_back((nr, nc));
        }
    }

    println!("{}", if visited[h - 1][w - 1] { "Yes" } else { "No" });
}
