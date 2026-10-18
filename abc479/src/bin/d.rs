use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・
* 【ACできたか？】
* ・
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・N <= 3 * 10^5
* ・O(N^2) は不可能
*
* 【Observation（問題から分かった事実）】
* ・クエリーが（1, 2）の時に、1→2に直接移動したものが最小値とすると、それはR[i-1] + 0が最小だったことになる
* ・L,Rは、今のマスにいればどれだけ移動してもコストは変わらない
* ・今回見るべきはコストのみで良いので、直接s→tに向かった時（Lのみ、もしくはRのみ）と、L、Rの最小値を比較すれば良さそう
* ・
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
1.直接s→tに向かった時を判定したいので、s > tであるか、もしくはs < tであるか
2.まずは、1でLのみ、Rのみのコストを確認する
3-1.2の逆パターンでの最小値を計算する（RもしくはLのコストは固定しているので、反対側のコストの最小値を見つける）
3-2.自分の今の位置をcurrentとした時、Rを求めるときは、幅はcurrent-1 <= i <= N-1（0-indexed）
3-2.自分の今の位置をcurrentとした時、Lを求めるときは、幅は0 <= i <= current-1（0-indexed）
3-3.区間での最小値をどのように求めるべきか？
3-4.n-1方面から最小値を求めていく
*
* 【計算量】
* ・O(N + Q)
*/

fn main() {
    input! {
        n: usize,
        q: usize,
        l_input: [usize; n-1],
        r_input: [usize; n-1],
    }

    let mut l = vec![usize::MAX; n];
    let mut r = vec![usize::MAX; n];

    for i in 1..n {
        l[i] = l_input[i - 1];
    }

    for i in 0..n - 1 {
        r[i] = r_input[i];
    }

    let mut prefix_r = r.clone();

    for i in 1..n {
        prefix_r[i] = prefix_r[i].min(prefix_r[i - 1]);
    }

    let mut suffix_l = l.clone();

    for i in (0..n - 1).rev() {
        suffix_l[i] = suffix_l[i].min(suffix_l[i + 1]);
    }

    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
        }

        let s = s - 1;
        let t = t - 1;

        let ans = if s < t {
            // 直接右へ
            let mut ans = r[s];

            // 一度左へ行ける場合
            if s > 0 {
                ans = ans.min(l[s] + prefix_r[s - 1]);
            }

            ans
        } else {
            // 直接左へ
            let mut ans = l[s];

            // 一度右へ行ける場合
            if s + 1 < n {
                ans = ans.min(r[s] + suffix_l[s + 1]);
            }

            ans
        };

        println!("{}", ans);
    }
}
