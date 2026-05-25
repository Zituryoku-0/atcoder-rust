use ascii::AsciiChar::B;
use ascii::AsciiChar::J;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

// #[path = "../../../lib/lib.rs"]
// mod lib;

fn main() {
    input! {
        x: usize,
        q: usize,
    }

    // low: 中央値以下の値を入れる
    // 最大値をすぐ取り出したいので普通のBinaryHeap
    let mut low: BinaryHeap<usize> = BinaryHeap::new();

    // high: 中央値より大きい値を入れる
    // 最小値をすぐ取り出したいのでReverseを使う
    let mut high: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    low.push(x);

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
        }

        add(a, &mut low, &mut high);
        add(b, &mut low, &mut high);

        println!("{}", low.peek().unwrap());
    }
}

fn add(value: usize, low: &mut BinaryHeap<usize>, high: &mut BinaryHeap<Reverse<usize>>) {
    if low.peek().map_or(true, |&median| value <= median) {
        low.push(value);
    } else {
        high.push(Reverse(value));
    }

    // lowが大きすぎる場合、最大値をhighに移す
    if low.len() > high.len() + 1 {
        let v = low.pop().unwrap();
        high.push(Reverse(v));
    }

    // highの方が多くなった場合、最小値をlowに移す
    if low.len() < high.len() {
        let Reverse(v) = high.pop().unwrap();
        low.push(v);
    }
}
