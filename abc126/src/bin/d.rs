use petgraph::visit;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::vec;

fn main() {
    input! {
        n: usize,
    }

    // 各頂点の行き先とその重みを管理するグラフ
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];

    // 各頂点をすでに訪れたかを管理する
    let mut visited: Vec<bool> = vec![false; n];

    // 各頂点の色を管理する（0:白、1:黒）
    let mut color: Vec<usize> = vec![0usize; n];

    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize, // 行き先の頂点
            w: usize, // 行き先までの重み
        }
        let u = u - 1;
        let v = v - 1;

        // 両方の頂点に追加する
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    // 最初の頂点は確定しているので、visitedをtrueにする
    visited[0] = true;
    // color[0] = 0;

    let mut stack: Vec<usize> = Vec::with_capacity(n);
    stack.push(0);

    while let Some(v) = stack.pop() {
        // その頂点に追加されている行き先の頂点の数だけ処理する
        for &(to, w) in &graph[v] {
            // すでに訪れていたらスキップ
            if visited[to] {
                continue;
            }

            stack.push(to);

            // 訪れたことを更新
            visited[to] = true;

            // 距離に応じて色を更新する
            if w % 2 == 0 {
                color[to] = color[v];
            } else {
                color[to] = 1 - color[v];
            }
        }
    }

    for col in color {
        println!("{}", col);
    }
}
