use petgraph::visit;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn dfs(
    grid: &mut Vec<Vec<usize>>,
    stack: &mut Vec<(usize, usize)>,
    n: usize,
    a: usize,
    b: usize,
    ans: &mut Vec<char>,
    count: &mut i32,
) {
    let mv = vec![(0 as i32, -1 as i32), (0, 1), (-1, 0), (1, 0)];
    let mvl = vec!['L', 'R', 'U', 'D'];
    while let Some((u, v)) = stack.pop() {
        if grid[u][v] == 1 {
            continue;
        }
        for i in 0..mv.len() {
            let ux = u as i32 + mv[i].0;
            let vy = v as i32 + mv[i].1;
            // gridの範囲を超えるならスキップ
            if ux < 0 || ux >= n as i32 || vy < 0 || vy >= n as i32 {
                continue;
            }
            stack.push((ux as usize, vy as usize));
            // visited状態にする
            grid[ux as usize][vy as usize] = 1;

            // マスが(n,n)でかつ、移動回数がn^2-2であればYes
            if u == n && v == n && *count == (n ^ 2 - 2) as i32 {
                println!("Yes");
                for i in 0..ans.len() {
                    print!("{}", ans[i]);
                }
                println!();
                return;
            }

            // a,bのセルでない
            if ux as usize != a && vy as usize != b {
                ans.push(mvl[i]);
                dfs(grid, stack, n, a, b, ans, count);
            }
            ans.pop();
            *count -= 1;
            // グリッドを戻す
            grid[ux as usize][vy as usize] = 0;
        }
    }
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: usize,
            b: usize,
        }

        let mut ans: Vec<char> = Vec::new();

        let mut grid: Vec<Vec<usize>> = vec![vec![0; n]; n];

        let mut stack: Vec<(usize, usize)> = Vec::new();

        let mut count = 0;

        // 最初の位置をvisited状態にする
        // grid[0][0] = 1;

        stack.push((0, 0));

        dfs(&mut grid, &mut stack, n, a, b, &mut ans, &mut count);
        println!("No");
    }
}
