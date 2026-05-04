use petgraph::visit::GraphProp;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec;

fn main() {
    input! {
        n: usize,
        d: usize,
    }
    let d = d as f64;
    let mut humans: Vec<(f64, f64)> = Vec::new();
    let mut virus: Vec<bool> = vec![false; n];
    for _ in 0..n {
        input! {
            x: i32,
            y: i32,
        }
        let x = x as f64;
        let y = y as f64;
        humans.push((x, y));
    }
    // 最初の人は感染している
    virus[0] = true;

    let mut graph: Vec<Vec<(f64, f64)>> = vec![humans; n];
    // println!("{:?}", graph);

    for i in 0..n {
        let (infect_x, infect_y) = graph[i][i];
        // println!("infect_xの値：{}", infect_x);
        // println!("infect_yの値：{}", infect_y);
        for j in 0..n {
            // 感染している人は飛ばす
            if virus[j] {
                continue;
            }
            if ((infect_x - graph[i][j].0).powf(2.0) + (infect_y - graph[i][j].1).powf(2.0)).sqrt()
                <= d
            {
                // println!("virusに追加する距離: {} {}", graph[i][j].0, graph[i][j].1);
                // println!(
                //     "この時の平方根：{}",
                //     ((infect_x - graph[i][j].0).powf(2.0) + (infect_y - graph[i][j].1).powf(2.0))
                //         .sqrt()
                // );
                virus[j] = true;
            }
        }
    }

    for vir in virus {
        if vir {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
