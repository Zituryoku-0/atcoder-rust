use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        n: usize,
        stages: [(usize, usize, usize);n-1],
    }

    // graph[i] = [(移動先, コスト), 、、、]
    let mut graph = vec![Vec::new(); n];

    for i in 0..n - 1 {
        let (a, b, x) = stages[i];

        graph[i].push((i + 1, a));
        graph[i].push((x - 1, b));
    }

    let mut dist = vec![usize::MAX; n];

    // (スタートからの距離, 頂点)
    let mut heap = BinaryHeap::new();

    dist[0] = 0;
    heap.push(Reverse((0usize, 0usize)));

    while let Some(Reverse((current_dist, current))) = heap.pop() {
        // 古い情報なら無視
        if current_dist > dist[current] {
            continue;
        }

        for &(next, cost) in &graph[current] {
            let next_dist = current_dist + cost;

            if next_dist < dist[next] {
                dist[next] = next_dist;

                heap.push(Reverse((next_dist, next)));
            }
        }
    }

    println!("{}", dist[n - 1]);
}
