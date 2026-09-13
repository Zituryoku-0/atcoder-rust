use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::writeln;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a = vec![0usize; n];

    // 現在A[i] > 0であるiを保持
    let mut positive = Vec::new();

    // 配列全体のXOR
    let mut xor_sum = 0usize;

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                x: usize,
            }

            let x = x - 1;

            // 0 -> 1になるなら、新たにpositiveを追加
            if a[x] == 0 {
                positive.push(x);
            }

            let old = a[x];
            let new = old + 1;

            // XORの差分更新
            xor_sum ^= old ^ new;

            a[x] = new;
        } else {
            let mut next_positive = Vec::new();

            for &i in &positive {
                let old = a[i];
                let new = old - 1;

                // XORの差分更新
                xor_sum ^= old ^ new;

                a[i] = new;

                // まだ1以上なら次回も更新対象
                if a[i] > 0 {
                    next_positive.push(i);
                }
            }
            positive = next_positive;
        }
        writeln!(out, "{}", xor_sum).unwrap();
    }
}
