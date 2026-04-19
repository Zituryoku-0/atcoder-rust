use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }

    // 最小ヒープ
    let mut min_heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut sum = 0_i64;

    for _ in 0..q {
        input! {
            category: i32,
        }
        match category {
            1 => {
                input! {
                    write_num: i64,
                }
                // 追加時に今までの加算分で引くことで、加算後のヒープとして整理できる
                min_heap.push(Reverse(write_num - sum));
            }
            2 => {
                input! {
                    add_num: i64,
                }
                // 今までの加算分をまとめる
                sum += add_num;
            }
            // それ以外のパターン（今回は3のみ）
            _ => {
                if let Some(Reverse(ans)) = min_heap.pop() {
                    // 追加時にマイナスしている分を戻して加算値を含めた数値として出力する
                    println!("{:?}", ans + sum);
                }
            }
        }
    }
}
