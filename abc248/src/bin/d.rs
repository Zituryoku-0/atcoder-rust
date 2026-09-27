use proconio::{input, marker::Chars};
use rand_core::le;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・約33分
* 【ACできたか？】
* ・できた！
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・Q <= 2 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・Xがクエリ事に変わるので、Xを効率よく判定する方法を考える必要がある
* ・Ai <= 2 * 10^5なので、各整数の個数を事前にVecで管理してもよさそうかなと思ったけど、区間に対する個数を判定することができないから、あまり意味がなさそう
* ・各整数の出現位置を管理すればよさそう（2次元Vec）
* ・そのVecに対して区間L, Rを満たす最小インデックスを二分探索で求め、そこからRまでの個数を求める
*
* 【状態・操作の整理】
* ・何が変化するか：
* ・何が変化しないか：
* ・答えを決めるために必要な情報：
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
*
* 【解法】
* ・上記まで整理すると、〜で解ける
*
* 【計算量】
* ・O(N + QlogN)
*/

fn main() {
    input! {
    n: usize,
    a: [usize; n],
    q: usize,
    }

    // 各整数のインデックスを管理する
    let mut indexes: Vec<Vec<usize>> = vec![Vec::new(); n];

    for i in 0..n {
        let idx: usize = a[i] - 1;
        indexes[idx].push(i);
    }

    for i in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        }

        let l = l - 1;
        let r = r - 1;
        let x = x - 1;

        let mut left: i32 = -1;
        let mut right = *&indexes[x].len();

        // leftを求める二分探索
        while right as i32 - left > 1 {
            let mid = (left + right as i32) / 2;
            if indexes[x][mid as usize] < l {
                left = mid;
            } else {
                right = mid as usize;
            }
        }

        let match_left = left;
        // println!("最終的なmatch_left{}", match_left);

        // rightを求める二分探索
        let mut left: i32 = -1;
        let mut right = *&indexes[x].len();

        // rightを求める二分探索
        while right as i32 - left > 1 {
            let mid = (left + right as i32) / 2;
            if indexes[x][mid as usize] > r {
                right = mid as usize;
            } else {
                left = mid;
            }
        }

        let match_right = right;
        // println!("最終的なmatch_right{}", match_right);

        println!("{}", match_right as i32 - match_left - 1);
    }
}
