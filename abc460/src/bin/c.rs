use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort();
    b.sort();

    let mut pop_a = a.pop();
    let mut ans = 0;
    let mut check = false;

    for _ in 0..m {
        if check {
            pop_a = a.pop();
        }
        let pop_b = b.pop();
        if let (Some(pop_a), Some(pop_b)) = (pop_a, pop_b) {
            // println!("pop_aの値：{}", pop_a);
            // println!("pop_bの値：{}", pop_b);
            if 2 * pop_a >= pop_b {
                check = true;
                ans += 1;
            } else {
                check = false;
            }
        } else {
            // 片方でNoneがあれば処理を終了
            break;
        }
    }
    println!("{}", ans);
}
