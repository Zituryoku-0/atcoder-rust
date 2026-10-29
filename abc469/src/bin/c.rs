use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;
use std::vec;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・
* 【ACできたか？】
* ・
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・N <= 8 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・持っている袋の当たりが無くなれば終わりなので、区間に対するあたりの数を管理しておく
* ・所持している当たりの袋の数だけ、先頭からお菓子を取れるので、今までに受け取っている袋 + 所持している当たりの袋の数 < 次のあたりの袋がある場所ならば、次のあたりの袋がある場所-1が答えになる
* ・今までに受け取っている袋 + 所持している当たりの袋の数 < 次のあたりの袋がある場所ならば、次のあたりの袋がある場所+1までのびる

・xの個数は減ることはないので、xを起点に考える
・x >= kとなるlが答えになる
・逆に、x < kの場合、答えは必ずnになる
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
* ・O(n)
*/

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let chrs: Vec<char> = s.chars().collect();
    let mut x_places = Vec::new();
    // xの場所を管理する
    for (i, c) in chrs.into_iter().enumerate() {
        let idx = i + 1;
        if c == 'x' {
            x_places.push(idx);
        }
    }

    let mut len = x_places.len();

    // xの場所がわかった
    for i in 1..=n {
        // k >= xの最大個数の場合は最大が答え
        if len == 0 || i > len {
            println!("{}", n);
            continue;
        }

        println!("{}", x_places[i - 1]);
    }
}
