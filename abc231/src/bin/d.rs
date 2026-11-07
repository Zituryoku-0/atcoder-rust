use nalgebra::convert_ref;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 * 解説
 * 条件(A_i, B_i)を辺として考えると、最終的にグラフ全体が「パスの集まり」になっていればYesである
 * つまり必要なのは、各頂点の字数が2以下かつ閉路が存在しないことである
 *
 * ・問題の本質
 * グラフとして考えると整理しやすい
 * 許される形は
 * 1 - 2 - 3 - 4
 * や
 * 1 - 2
 * 3 - 4 - 5
 * 6
 * のような、パスと孤立頂点の集合である
 *
 * 逆に許されない形は
 * 1.次数が3以上である
 * 2.閉路である
 *
 * したがって、判定条件は以下2つである
 * 1.全頂点について、degree <= 2
 * 2.閉路が存在しない
 *
 *
 *
 *
 *
 *
 *
 *
 */

fn dfs(current: usize, parent: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> bool {
    visited[current] = true;

    /**
     * この処理は、DFSを使って、無向グラフに閉路が存在するかを判定するロジック
     *
     * 特に重要なのは、以下の3つ
     * 1.next == parent ：単に来た道を戻るだけなので無視する
     * 2.visited[next]：親以外の訪問済み頂点に到達したら平路と判断する
     * 3.dfs(next, current, ...)：次の頂点へ移動し、さらに深く探索する
     *
     * 1.if next == parentが必要な理由
     * 無向グラフの場合、辺は双方向につながっている
     * 例： 1 --- 2であれば
     * graph[0] = vec![1];
     * graph[1] = vec![0];となっている
     *
     * そのため、頂点1から頂点2へ移動する時のDFSの流れは
     * dfs(0, 4) → dfs(1, 0)となる
     * ここで、頂点2を探索すると
     * current = 1, parent = 0, next = 0である
     * つまり、頂点2から見た隣接頂点には、直前にいた頂点1が含まれている
     * そのため、もし親を無視しなければ
     * if visited[next] {
     *   return false;
     * }
     * として、頂点1が訪問済みと判定され、閉路が存在しないのに閉路があると誤判定してしまう
     *
     * 2.if visited[next]で閉路を検出できる理由
     * if visited[next] {
     *   return false;
     * }
     * この条件が意味するのは、「親以外の頂点なのに、すでに訪問済みである」ということである
     *
     * 例えば、1 - 2 - 3 - 4 - 1と繋がる閉路の場合
     * 頂点1から始めると
     * dfs(1) → dfs(2) → dfs(3) → dfs(4)となる
     * ここで、頂点4を探索すると、隣接頂点は頂点3と頂点1である
     * ・頂点3は親なのでcontinue
     * ・頂点1は親ではない
     * ・頂点1はすでに訪問済みのため、閉路と判定できる
     *
     * 3.if !dfs(next, current, ...)の意味
     * if !dfs(next, current, graph, visited){
     *   return false;
     * }
     * これは、次の頂点へdfsで進み、その先で閉路が見つかったら呼び出しもとにもfalseを伝える処理である
     *
     *
     */
    for &next in &graph[current] {
        // 来た道を戻るだけなら問題ない
        if next == parent {
            continue;
        }

        // 親以外の訪問済み頂点に到達した → 閉路
        if visited[next] {
            return false;
        }

        if !dfs(next, current, graph, visited) {
            return false;
        }
    }

    true
}
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![Vec::new(); n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }

        let a = a - 1;
        let b = b - 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    // 次数が3以上なら横一列にはできない
    for i in 0..n {
        if graph[i].len() > 2 {
            println!("No");
            return;
        }
    }

    let mut visited = vec![false; n];

    // 連結成分が複数存在する可能性があるため
    // 全頂点を開始点候補として確認する
    for i in 0..n {
        if visited[i] {
            continue;
        }

        if !dfs(i, n, &graph, &mut visited) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
