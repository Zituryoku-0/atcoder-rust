use nalgebra::VectorSliceMut1;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn diff_one(a: &Vec<char>, b: &Vec<char>) -> bool {
    let mut diff = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            diff += 1;
        }
    }
    diff == 1
}

fn next_permutation(v: &mut Vec<usize>) -> bool {
    let n = v.len();
    if n <= 1 {
        return false;
    }

    // 1.v[i] < v[i + 1]となる一番右のiを探す
    let mut i = n - 2;
    loop {
        if v[i] < v[i + 1] {
            break;
        }
        if i == 0 {
            return false;
        }
        i -= 1;
    }

    // 2.v[i] < v[j]となる一番右のjを探す
    let mut j = n - 1;
    while v[i] >= v[j] {
        j -= 1;
    }

    // 3. swap
    v.swap(i, j);

    // 4. i + 1以降を反転
    v[i + 1..].reverse();

    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut order: Vec<usize> = (0..n).collect();

    loop {
        let mut ok = true;
        for i in 0..n - 1 {
            if !diff_one(&s[order[i]], &s[order[i + 1]]) {
                ok = false;
                break;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
        if !next_permutation(&mut order) {
            break;
        }
    }
    println!("No");
}
