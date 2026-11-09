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
        q: usize,
        k: [usize; q],
    }

    let mut cards = Vec::with_capacity(n);

    for i in 0..n {
        cards.push((i + 1, true));
    }

    for i in 0..q {
        let mut temp = Vec::new();
        // 順序も全て変わる
        if k[i] == n {
            for j in (0..n).rev() {
                temp.push(cards[j]);
                let len = temp.len() - 1;
                if temp[len].1 {
                    temp[len].1 = false;
                } else {
                    temp[len].1 = true;
                }
            }
            cards = temp;
            continue;
        }

        let cut = cards[k[i]..n].to_vec();
        for j in (0..k[i]).rev() {
            temp.push(cards[j]);
            let len = temp.len() - 1;
            if temp[len].1 {
                temp[len].1 = false;
            } else {
                temp[len].1 = true;
            }
        }
        cards = temp;
        cards.extend(cut);
    }

    for i in 0..n {
        if cards[i].1 {
            println!("{}", cards[i].0);
        } else {
            println!("-{}", cards[i].0);
        }
    }
}
