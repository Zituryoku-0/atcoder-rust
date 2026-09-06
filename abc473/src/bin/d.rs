use ascii::AsciiChar::B;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Write as FmtWrite;
use std::io;
use std::io::Write as IoWrite;

#[path = "../../../lib/lib.rs"]
mod lib;

fn dfs(index: usize, sum: usize, n: usize, k: usize, a: &mut [usize], output: &mut String) {
    // 最後のA_Nだけは探索せず、数式で求める
    if index + 1 == n {
        let remaining = k - sum;

        // N * A_N = remaining
        // となる非負整数A_Nが存在するか
        if remaining % n == 0 {
            a[index] = remaining / n;

            for i in 0..n {
                if i > 0 {
                    output.push(' ');
                }
                write!(output, "{}", a[i]).unwrap();
            }
            output.push('\n');
        }
        return;
    }

    // indexは0-indexedなので
    // A[index]の係数はindex + 1
    let coefficient = index + 1;

    // sum + coefficient * x <= K
    // を満たす最大のx
    let max_x = (k - sum) / coefficient;

    // 小さい値から試すことで辞書順になる
    for x in 0..=max_x {
        a[index] = x;
        dfs(index + 1, sum + coefficient * x, n, k, a, output);
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut a = vec![0; n];

    // 解が30万個あるため、
    // println!をまとめて出力する
    let mut output = String::new();

    dfs(0, 0, n, k, &mut a, &mut output);

    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout.lock());
    write!(stdout, "{}", output).unwrap();
}
