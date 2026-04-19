use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }

    let mut graph = vec![Vec::<(usize, usize)>::new(); n];

    for _ in 0..n - 1 {
        input! {
            u:usize,
            v:usize,
            w:usize,
        }
        let u = u - 1;
        let v = v - 1;

        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let mut color = vec![0usize; n];
    let mut visited = vec![false; n];

    let mut stack = Vec::new();
    // 先頭は0で良い
    stack.push(0);
    visited[0] = true;

    while let Some(cur) = stack.pop() {
        for &(next, w) in &graph[cur] {
            // すでにuを訪れていたらスキップ
            if visited[next] {
                continue;
            }

            if w % 2 == 0 {
                color[next] = color[cur];
            } else {
                color[next] = 1 - color[cur];
            }

            visited[next] = true;
            stack.push(next);
        }
    }

    for c in color {
        println!("{}", c);
    }
}
