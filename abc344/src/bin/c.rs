use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

/****
 *
 * 解説の確認
 * 事前にa,b,cの全ての計算をHashSetに入れておき、q[i]がHashSetに存在するかを計算すればよかった
 * 1 <= N, M, L <= 100のため
 * 計算量はO(Q + NML)である
 */

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    }

    let mut set = HashSet::new();

    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                // 事前に全ての計算結果を入れておく
                set.insert(a[i] + b[j] + c[k]);
            }
        }
    }

    for i in 0..q {
        if set.contains(&x[i]) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
