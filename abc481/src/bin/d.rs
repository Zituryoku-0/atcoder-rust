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
        m: usize,
        s: usize,
        t: usize,
        edges: [(usize, usize); m],
    }

    // 0-indexed
    let s = s - 1;
    let t = t - 1;

    // graph[v] = vから直接行ける惑星
    let mut graph = vec![Vec::new(); n];

    // min_to[v] = vから行ける惑星のうち最小番号
    let mut min_to = vec![usize::MAX; n];

    for (a, b) in edges {
        let a = a - 1;
        let b = b - 1;

        graph[a].push(b);
        min_to[a] = min_to[a].min(b);
    }

    let inf = usize::MAX;

    // dist[v] = Sからvまでに必要な最小燃料
    let mut dist = vec![inf; n];

    let mut deque = VecDeque::new();

    dist[s] = 0;
    deque.push_front(s);

    while let Some(v) = deque.pop_front() {
        for &to in &graph[v] {
            // 最小番号の惑星なら燃料0
            // それ以外なら燃料1
            let cost = if to == min_to[v] { 0 } else { 1 };

            let next_dist = dist[v] + cost;

            if next_dist < dist[to] {
                dist[to] = next_dist;

                if cost == 0 {
                    // 距離が増えていないので優先的に調べる
                    deque.push_front(to);
                } else {
                    // 距離が1増えたので後ろに入れる
                    deque.push_back(to);
                }
            }
        }
    }

    if dist[t] == inf {
        println!("-1");
    } else {
        println!("{}", dist[t]);
    }
}
