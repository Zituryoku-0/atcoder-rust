use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: usize,
        mut a: [usize; n],
    }

    a.sort_unstable();

    // Tより後の青木くんは必要ない
    let a: Vec<usize> = a.into_iter().filter(|&x| x <= t).collect();

    // Tまでに青木くんが一度も押さない場合
    if a.is_empty() {
        let ans = (t + k - 1) / k;
        println!("{}", ans);
        return;
    }

    let mut ans = 0usize;

    // 0 ~最初のあおきくん
    ans += (a[0] + k - 1) / k;

    // 青木くん ~ 青木くん
    for i in 1..a.len() {
        let diff = a[i] - a[i - 1];

        if diff > 0 {
            // ceil(diff / k) - 1
            ans += (diff - 1) / k;
        }
    }

    // 最後の青木くん ~ T
    let diff = t - a[a.len() - 1];

    if diff > 0 {
        // ceil(diff / k) -1
        ans += (diff - 1) / k;
    }

    println!("{}", ans);
}
