use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
* 【問題にかかった時間（ACできたか問わない）】
* ・25分（考察で諦めた）
* 【ACできたか？】
* ・できなかった
* 【問題の目的】
* ・最終的に何を求める問題か
*
* 【制約から分かること】
* ・N <= 2 * 10^5
* ・O(N^2) は不可能

  ここから解説
*
* 【Observation（問題から分かった事実）】
* ・子供iに大きな飴をbi個配るとする。
* ・小さい雨はAi - bi個なので、子供iの総重量wは
* ・W = X(Ai - bi) + Ybi
      = XAi + (Y - X)bi
* ・従って bi = (W - XAi) / (Y - X)
* ・つまり全員の共通重量Wが決まれば、各子供の大きなアメの個数も一意に決まる
* ・子供iが取り得る重量は　XAi <= W <= YAi
* ・従って全員共通のWは max(XAi) <= W <= min(YAi)を満たす必要がある
* ・さらにbiは整数なので W - XAiが　Y-Xで割り切れる必要がある
* ・つまり全てのiについて XAi mod (Y-X)が同じでなければならない
* ・大きなアメの個数biはWが大きくなるほど増えるため、条件を満たす最大のWを選べば良い
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
* ・A[i]の最小を見るためにソートする
 ・
*
* 【計算量】
* ・O(N + logN + NlogN)
*/

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        a: [i64; n],
    }

    let diff = y - x;

    // 全員が実現できる共通重量Wの範囲
    let lower = a.iter().map(|&ai| x * ai).max().unwrap();
    let upper = a.iter().map(|&ai| y * ai).min().unwrap();

    // そもそも共通重量を作れない
    if lower > upper {
        println!("-1");
        return;
    }

    // W = x * ai + (y - x) * bi
    // よって、全員について
    // W ≡ x * ai (mod y - x)
    // が同じでなければならない
    let rem = (x * a[0]) % diff;

    for &ai in &a {
        if (x * ai) % diff != rem {
            println!("-1");
            return;
        }
    }

    // upper以下でW ≡ rem (mod diff)
    // を満たす最大のWを求める
    let mut w = upper;

    let current_rem = w % diff;

    if current_rem >= rem {
        w -= current_rem - rem;
    } else {
        w -= current_rem + diff - rem;
    }

    // 調整した結果、下限を下回ったら不可能
    if w < lower {
        println!("-1");
        return;
    }

    // 各人について
    // bi = (W - x * ai) / (y - x)
    // を求める
    let mut ans = 0_i64;

    for &ai in &a {
        let big = (w - x * ai) / diff;
        ans += big;
    }

    println!("{}", ans);
}
