use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        h : usize,
        w : usize,
        s : [Chars; h],
    }

    let mut ans = 0usize;

    for h1 in 0..h {
        for h2 in h1..h {
            for w1 in 0..w {
                for w2 in w1..w {
                    let mut ok = true;

                    for i in h1..=h2 {
                        for j in w1..=w2 {
                            let ni = h1 + h2 - i;
                            let nj = w1 + w2 - j;

                            if s[i][j] != s[ni][nj] {
                                ok = false;
                            }
                        }
                    }
                    if ok {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
