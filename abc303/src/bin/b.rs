use petgraph::graph;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut prev: Option<usize> = None;
    let mut graph: Vec<Vec<bool>> = vec![vec![false; n]; n];

    for _ in 0..m {
        for _ in 0..n {
            input! {
                a:usize,
            }
            let a = a - 1;
            if let Some(p) = prev {
                graph[p][a] = true;
                graph[a][p] = true;
            }
            prev = Some(a);
        }
    }

    let mut ans: usize = 0;
    for i in 0..n {
        for j in i + 1..n {
            if !graph[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
