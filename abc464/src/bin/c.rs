use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::println;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut colors = vec![0i32; n];
    let mut change_colors = Vec::new();
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            a: usize,
            d: usize,
            b: usize,
        }
        let a = a - 1;
        let d = d - 1;
        let b = b - 1;

        set.insert(a);
        colors[a] += 1;
        change_colors.push((d, a, b));
    }

    change_colors.sort();

    let mut day_count = 0;
    let mut idx = 0;
    for i in 0..m {
        let _start = idx;
        for j in _start..change_colors.len() {
            let (d, a, b) = change_colors[j];
            // dが一致しないなら出力
            if day_count != d {
                idx = j;
                break;
            }

            colors[a] -= 1;
            colors[b] += 1;
            // 色の変化によって0になった場合はsetから削除する
            if colors[a] < 1 {
                set.remove(&a);
            }
            // setには必ずinsertする、重複してたら何も起きないので問題なし
            set.insert(b);
            // println!("setの中身：{:?}", set);
        }
        println!("{}", set.len());
        day_count += 1;
    }
}
