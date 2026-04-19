use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![Vec::new(); n];
    let mut indeg = vec![0usize; n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        let a = a - 1;
        let b = b - 1;
        graph[a].push(b);
        indeg[b] += 1;
    }

    // 入次数0の頂点を最小番号から取りたいので最小ヒープ
    let mut heap = BinaryHeap::new();
    for v in 0..n {
        if indeg[v] == 0 {
            heap.push(Reverse(v));
        }
    }

    let mut ans = Vec::with_capacity(n);

    while let Some(Reverse(v)) = heap.pop() {
        ans.push(v);

        for &to in &graph[v] {
            indeg[to] -= 1;
            if indeg[to] == 0 {
                heap.push(Reverse(to));
            }
        }
    }

    if ans.len() != n {
        println!("{}", -1);
        return;
    }

    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans[i] + 1);
    }
    println!();
}
