use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::write;
use std::writeln;

/**
 *
 * 計算量はO(Q + 10^6)
 *
 *
 */
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut next = vec![None; n + 1];
    let mut prev = vec![None; n + 1];

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                x: usize,
                y: usize,
            }

            next[x] = Some(y);
            prev[y] = Some(x);
        } else if query_type == 2 {
            input! {
                x: usize,
                y: usize,
            }

            next[x] = None;
            prev[y] = None;
        } else {
            input! {
                x: usize,
            }

            // まず先頭まで見る
            let mut current = x;

            while let Some(p) = prev[current] {
                current = p;
            }

            // 先頭から末尾まで収集
            let mut ans = Vec::new();

            loop {
                ans.push(current);

                match next[current] {
                    Some(nxt) => current = nxt,
                    None => break,
                }
            }
            write!(out, "{}", ans.len()).unwrap();

            for train in ans {
                write!(out, " {}", train).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
