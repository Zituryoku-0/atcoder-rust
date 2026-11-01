use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        s: [Chars; 5],
    }

    let mut yoko = vec![0; 5];
    let mut tate = vec![0; 5];
    let mut ue_naname = 0;
    let mut shita_naname = 0;

    let mut ans = usize::MAX;

    for i in 0..5 {
        for j in 0..5 {
            if s[i][j] == '.' {
                tate[j] += 1;
                yoko[i] += 1;
                if i == j {
                    shita_naname += 1;
                }
                if (i == 4 && j == 0)
                    || (i == 3 && j == 1)
                    || (i == 2 && j == 2)
                    || (i == 1 && j == 3)
                    || (i == 0 && j == 4)
                {
                    ue_naname += 1;
                }
            }
        }
    }

    for i in 0..5 {
        ans = ans.min(tate[i]);
        ans = ans.min(yoko[i]);
        if i == 0 {
            ans = ans.min(shita_naname);
            ans = ans.min(ue_naname);
        }
    }

    println!("{}", ans);
}
