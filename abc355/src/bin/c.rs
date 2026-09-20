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
 * 解説
 * 行は x / nで特定できる
 * 列は x % nで特定できる
 * 右下の斜めは (0, 0), (1, 1), (2, 2)より
 * i == jで特定できる
 *
 * 右上の斜めは(2, 0), (1, 1), (0, 2)より
 * i + j == nで特定できる
 *
 * 各場所をO(1)で特定できるため、T回の処理の結果計算量はO(T)で求めることができる
 */

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; t],
    }

    let mut row_count = vec![0; n];
    let mut col_count = vec![0; n];

    let mut diag1 = 0;
    let mut diag2 = 0;

    for (turn, &value) in a.iter().enumerate() {
        // 1-indexedの数字を0-indexedに変換
        let x = value - 1;

        let row = x / n;
        let col = x % n;

        row_count[row] += 1;
        col_count[col] += 1;

        // 左上→右下
        if row == col {
            diag1 += 1;
        }

        // 右上→左下
        if row + col == n - 1 {
            diag2 += 1;
        }

        // 今回更新した行・列・対角線だけ確認すれば良い
        if row_count[row] == n || col_count[col] == n || diag1 == n || diag2 == n {
            println!("{}", turn + 1);
            return;
        }
    }
    println!("-1");
}
