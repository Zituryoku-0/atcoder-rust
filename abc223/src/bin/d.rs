use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // グラフ
    let mut graph = vec![vec![]; n];
    // 入次数の管理
    let mut indeg: Vec<usize> = vec![0_usize; n];
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

    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    // 最初の入次数が0のものを取得する
    for i in 0..n {
        if indeg[i] == 0 {
            heap.push(Reverse(i));
        }
    }

    let mut ans = Vec::with_capacity(n);
    // ヒープが無くなるまでループ
    while let Some(Reverse(v)) = heap.pop() {
        ans.push(v);

        for &to in &graph[v] {
            indeg[to] -= 1;
            if indeg[to] == 0 {
                heap.push(Reverse(to));
            }
        }
    }

    if ans.len() == n {
        for i in 0..n {
            if i > 0 {
                print!(" ");
            }
            print!("{:?}", ans[i] + 1);
        }
    } else {
        print!("-1");
    }
    // println!();
}
