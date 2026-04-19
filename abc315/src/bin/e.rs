use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n]; // グラフ
    let mut indeg: Vec<usize> = vec![0; n]; // 入次数
    for i in 0..n {
        input! {
            c: usize,
        }
        for _ in 0..c {
            input! {
                p: usize,
            }
            let p = p - 1;
            graph[p].push(i);
            indeg[i] += 1;
        }
    }

    let mut que: VecDeque<usize> = VecDeque::new();
    // 最初の入次数が0のものを入れる
    for i in 0..n {
        if indeg[i] == 0 {
            que.push_back(i);
        }
    }

    // println!("{:?}", graph);
    // println!("{:?}", indeg);

    let mut ans: VecDeque<usize> = VecDeque::new();
    // キューの中身が無くなるまで
    while let Some(v) = que.pop_front() {
        let mut count = 0;
        for &to in &graph[v] {
            indeg[to] -= 1;
            count += 1;
            if indeg[to] == 0 {
                // 入次数が0になればキューに追加
                que.push_back(to);
            }
        }
        if count > 0 {
            ans.push_back(v);
        }
        // println!("入次数：{:?}", indeg);
        if indeg[0] == 0 {
            break;
        }
    }

    while let Some(v) = ans.pop_front() {
        print!("{}", v + 1);
        if !ans.is_empty() {
            print!(" ");
        }
    }
    println!()
}
