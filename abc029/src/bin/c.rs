use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/**
 *
 * 辞書順のDFS
 * wordsは['a', 'b', 'c']と決まっている
 * forはnだけ行う
 * 必要なもの
 * n（ループ最大回数）、words（文字）、counter（現在のループ回数）、
 * index == n-1になればループは終わり
 * 多重ループする前に、今の文字を持っておく必要がある
 * aをStringに文字をpushしていき、出力後にpopする（len == nの時）
 */

fn dfs(n: usize, words: [char; 3], output: &mut String) {
    // 文字数 == nのとき
    if output.len() >= n - 1 {
        for i in 0..3 {
            output.push(words[i]);
            println!("{}", output);
            output.pop();
        }
        return;
    }

    for i in 0..3 {
        output.push(words[i]);
        dfs(n, words, output);
        output.pop();
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut output = String::new();
    let words = ['a', 'b', 'c'];

    dfs(n, words, &mut output);
}
