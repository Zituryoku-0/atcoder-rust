use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn dfs(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
    visited[v] = true;
    ans.push(v);

    for &to in &graph[v] {
        if visited[to] {
            continue;
        }
        dfs(to, graph, visited, ans);
        ans.push(v); // 子から戻ってきた頂点を記録
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut graph = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        input! {
            a:usize,
            b:usize,
        }
        let a = a - 1;
        let b = b - 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    for i in 0..n {
        // 各頂点が持つ要素を昇順にソートする
        graph[i].sort_unstable();
    }

    let mut visited = vec![false; n];
    let mut ans = Vec::with_capacity(2 * n - 1);

    dfs(0, &graph, &mut visited, &mut ans);

    for i in 0..ans.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans[i] + 1);
    }
    println!();
}
