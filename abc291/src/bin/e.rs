use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![vec![]; n];
    let mut indeg = vec![0usize; n];

    for _ in 0..m {
        input! {
            x: usize,
            y: usize,
        }
        let x = x - 1;
        let y = y - 1;

        graph[x],push(y);
        indeg[y] += 1;
    }

    let mut que = VecDeque::new();
    for i in 0..n {
        if indeg[i] == 0 {
            que.push_back(i);
        }
    }

    let mut order = Vec::with_capacity(n);

    while !que.is_empty() {
        // 入次数0の頂点が複数ある -> 順序は一位ではない
        if que.len() >= 2 {
            println!("No");
            return;
        }

        let v = que.pop_front().unwrap();
        order.push(v);

        // 入次数が0である頂点の次の頂点を確認する
        for &to in &graph[v] {
            indeg[to] -= 1;
            if indeg[to] == 0 {
                que.push_back(to);
            }
        }
    }

    // 問題文より矛盾しないAは存在するが、保険として書いておいて良い
    if order.len() != n {
        println!("No");
        return;
    }

    // ans[i] = A_i
    let mut ans = vec![0usize; n];
    for (i, &v) in order.iter().enumerate() {
        ans[v] = i + 1;
    }

    println!("Yes");

    for i in 0..n {
        if i > 0{
            print!(" ");
        }
        print!("{}", ans[i]);
    }
    println!();
}
