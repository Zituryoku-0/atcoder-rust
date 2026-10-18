use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::write;
use std::writeln;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize, usize); q],
    }

    // ranges[x]：値xを追加する区間の一覧
    let mut ranges = vec![Vec::<(usize, usize)>::new(); q + 1];

    for (l, r, x) in queries {
        ranges[x].push((l, r));
    }

    // r + 1がn + 1になる場合に備えて n + 2要素用意する
    // 減算を記録するので符号付き整数を使う
    let mut diff = vec![0_i64; n + 2];

    for intervals in ranges.iter_mut() {
        if intervals.is_empty() {
            continue;
        }

        // タプルは、第一要素→第二要素の順に比較される
        // したがって左端の昇順になる
        intervals.sort_unstable();

        let (mut left, mut right) = intervals[0];

        for &(l, r) in intervals.iter().skip(1) {
            if l <= right {
                // 現在の区間と重なっているので統合する
                right = right.max(r);
            } else {
                // 現在の区間を確定する
                diff[left] += 1;
                diff[right + 1] -= 1;

                // 次の区間をまとめ始める
                left = l;
                right = r;
            }
        }

        // 最後の区間はループ内では確定されないので、ここで記録する
        diff[left] += 1;
        diff[right + 1] -= 1;
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    // 差分配列の累積和が、それぞれの集合の要素数になる
    let mut count = 0i64;

    for i in 1..=n {
        count += diff[i];

        if i > 1 {
            write!(out, " ").unwrap();
        }
        write!(out, "{}", count).unwrap();
    }

    writeln!(out).unwrap();
}
