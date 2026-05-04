use proconio::{input, marker::Chars};
use rand_core::le;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::vec;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![0; n]; n];
    let mut temp: VecDeque<usize> = VecDeque::new();

    for _ in 0..m {
        temp.clear();
        for _ in 0..n {
            input! {
                a: usize,
            }

            let a = a - 1;

            if let Some(v) = temp.pop_front() {
                temp.push_back(a);
                graph[v][a] += 1;
                graph[a][v] += 1;
            } else {
                temp.push_back(a);
                continue;
            }
        }
    }

    let mut ans = 0;

    for i in 0..n {
        for j in i + 1..n {
            if graph[i][j] == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
