use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }

    let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();

    for _ in 0..q {
        input! {
            t: i32,
            h: i64,
        }

        if t == 1 {
            // 高さhの木を追加
            heap.push(Reverse(h));
        } else {
            // 高さh以下の木を全て削除
            while let Some(&Reverse(x)) = heap.peek() {
                if x <= h {
                    heap.pop();
                } else {
                    break;
                }
            }
        }
        println!("{}", heap.len());
    }
}
