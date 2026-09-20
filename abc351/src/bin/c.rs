use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・約23分
* 【ACできたか？】
* ・できた
* 【問題の目的】
* ・最終的に何を求める問題か
Nこのボールを順番に追加し、同じ大きさのボールが右端に2個ならぶ限り合体させる
最終的に残るボール数を求める
*
* 【制約から分かること】
* ・N <= 2 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・基本的に末尾と末尾-1を判定する
* ・処理順的にstackで管理するとやりやすそう
* ・連続して大きさが同じボールが来たら、1つボールをpopして、もう一つの方に+1すれば良い
*
* 【状態・操作の整理】
* ・何が変化するか：
* ・何が変化しないか：
* ・答えを決めるために必要な情報：
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
・連続して同じ大きさのボールが並ばない状態で、個数は何個ありますかと言い換えれる
*
* 【解法】
* ・上記まで整理すると、〜で解ける
・stackのlenが1の時は何もしない
・ループの条件としては、while stack.len() > 1みたいな感じ
・stackのlenが2以上で末尾と末尾-1に対して処理を行う
・末尾と末尾-1で大きさが一致すれば、2つのボールをpopして、2^Ai + 2したものを末尾にpushする
*
* 【計算量】
* ・O(N)
合体1回につきstackの要素数が1減るため、合体回数は全体で高々N-1回
*/

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut stack = Vec::with_capacity(n);

    for i in 0..n {
        stack.push(a[i]);
        while stack.len() > 1 {
            let len = stack.len() - 1;
            if stack[len] == stack[len - 1] {
                // ここはわざわざ2回popしなくて良い
                match stack.pop() {
                    Some(_) => stack[len - 1] += 1,
                    None => return,
                }
            } else {
                break;
            }
        }
    }

    println!("{}", stack.len());
}
