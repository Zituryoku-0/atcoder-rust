use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;
use std::writeln;

/**
 * タイルが置かれている間は、色が変わらない
 * 特にマスに色を塗る作業がO(1)で終わらせたい
 *
 *
 *
 *
 *
 */

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut colors = Vec::new();
    // 初期の色
    colors.push('a');
    let mut mas: Vec<i64> = vec![-1; n];
    let mut tyle = vec![false; n];
    let mut tyle_out = VecDeque::new();

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for i in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                x: usize,
            }
            let x = x - 1;
            if tyle[x] {
                tyle[x] = false;
                tyle_out.push_back(x);
            } else {
                // タイルを被せた時は、masにその時点でのtyleの最新を設定しておく
                tyle[x] = true;
                if mas[x] == -1 {
                    mas[x] = (colors.len() - 1) as i64;
                }
            }
        } else {
            input! {
                c: char,
            }
            colors.push(c);
            while let Some(v) = tyle_out.pop_front() {
                if !tyle[v] {
                    mas[v] = -1;
                }
            }
        }
    }

    for i in 0..n {
        if mas[i] == -1 {
            write!(out, "{}", colors[colors.len() - 1]).unwrap();
        } else {
            write!(out, "{}", colors[mas[i] as usize]).unwrap();
        }
    }
    writeln!(out).unwrap();
}
