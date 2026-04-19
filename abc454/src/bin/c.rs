use petgraph::graph;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::vec;

fn dfs(
    v: usize,
    graph: &Vec<Vec<usize>>,
    ans: &mut usize,
    count: &mut usize,
    visited: &mut Vec<bool>,
) {
    visited[v] = true;
    *count += 1;
    *ans = *ans.max(count);
    for &to in &graph[v] {
        if !visited[to] {
            dfs(to, graph, ans, count, visited);
        }
    }
    // *count -= 1;
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut at: Vec<usize> = Vec::new();
    let mut ans: usize = 0;
    let mut count: usize = 0;
    let mut visited: Vec<bool> = vec![false; n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        let a = a - 1;
        let b = b - 1;
        graph[a].push(b);
    }
    at.push(0);

    dfs(0, &graph, &mut ans, &mut count, &mut visited);

    println!("{}", ans);
}
