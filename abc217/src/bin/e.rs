use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }

    let mut queue: VecDeque<i64> = VecDeque::new();
    let mut min_heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();

    for _ in 0..q {
        input! {
            query: i64,
        }

        match query {
            1 => {
                input! {
                    x: i64,
                }
                queue.push_back(x);
            }
            2 => {
                if let Some(Reverse(x)) = min_heap.pop() {
                    println!("{}", x);
                } else {
                    let x = queue.pop_front().unwrap();
                    println!("{}", x);
                }
            }
            _ => {
                while let Some(x) = queue.pop_front() {
                    min_heap.push(Reverse(x));
                }
            }
        }
    }
}
