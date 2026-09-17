use im_rc::HashMap;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::print;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
* 【問題の目的】
* ・最終的に何を求める問題か
・Si = SjとTi = Tjとなる素数を出力する問題
*
* 【制約から分かること】
* ・N <= 7
* ・O(N^2) は可能
*
* 【Observation（問題から分かった事実）】
* ・7桁の素数を洗い出す必要がある（エラストてネスの篩）
* ・文字が同じものには同じ数字である素数を出力する（無ければ-1）
* ・
*
* 【状態・操作の整理】
* ・何が変化するか：
* ・何が変化しないか：
・素数の個数
* ・答えを決めるために必要な情報：
・入力の文字と数字のマッピングが必要（例えば、「c」のとき「1」なら、それを保持する）
・素数を管理する（Vecで素数にはtrue、そうでないものにはfalseをつければ良さそう）
*
* 【問題の言い換え】
* ・元の問題は「〜」と考え直せる
*
* 【解法】
* ・上記まで整理すると、〜で解ける
・素数を洗い出す（エラストてネスの篩）
・先頭からdfsで一致する素数が存在するかを洗い出す
*
* 【計算量】
* ・O(N^7logN) N <= 7なので、間に合いそう？
*/

fn dfs(chrs: &[char], mut ans: usize, len: usize, mut current: usize, prime: &[bool]) {
    if current == len {
        if prime[ans - 1] {
            println!("{}", ans);
            return;
        }
    }

    for i in 1..=len {
        for j in 1..=9 {
            ans += j * current;
            dfs(chrs, ans, len, current + 1, prime);
        }
    }

    println!("-1");
    return;
}

fn main() {
    input! {
        s: String,
    }

    let max_num = 10_000_000;
    let mut prime = vec![true; max_num];
    let mut chrs: Vec<char> = s.chars().collect();
    let mut ans = 0;
    let len = s.len();
    // 1は素数でない
    prime[0] = false;
    // 素数を列挙する
    loop {
        for i in 1..=max_num {
            let prime_num = i - 1;
            if prime[prime_num] {
                for j in 1..max_num {
                    if prime_num * j > max_num {
                        break;
                    }
                    prime[prime_num * j] = false;
                }
            }
        }
        break;
    }

    dfs(&chrs, ans, len, 0, &prime);
}
