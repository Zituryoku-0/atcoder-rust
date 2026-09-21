use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

/**
 * セグ木を初めて実装してみる
 * セグ木を使うシチュエーションはなんとなく理解できた
 * 静的な配列：累積和を使えば良い
 * 配列の要素が更新される場合：セグ木を使うのが良い
 */

#[derive(Clone, Copy)]
struct Node {
    min_value: usize,
    min_index: usize,
    max_value: usize,
    max_index: usize,
}

struct SegTree {
    size: usize,
    tree: Vec<Node>,
}

impl SegTree {
    fn new(p: &[usize]) -> Self {
        let n = p.len();

        let mut size = 1;
        while size < n {
            size *= 2;
        }

        let inf = usize::MAX;

        let mut tree = vec![
            Node {
                min_value: inf,
                min_index: 0,
                max_value: 0,
                max_index: 0,
            };
            size * 2
        ];

        // 葉をセット
        for i in 0..n {
            tree[size + i] = Node {
                min_value: p[i],
                min_index: i,
                max_value: p[i],
                max_index: i,
            };
        }

        // 下から構築
        for i in (1..size).rev() {
            tree[i] = Self::merge(tree[i * 2], tree[i * 2 + 1]);
        }

        Self { size, tree }
    }

    fn merge(left: Node, right: Node) -> Node {
        let (min_value, min_index) = if left.min_value < right.min_value {
            (left.min_value, left.min_index)
        } else {
            (right.min_value, right.min_index)
        };

        let (max_value, max_index) = if left.max_value > right.max_value {
            (left.max_value, left.max_index)
        } else {
            (right.max_value, right.max_index)
        };

        Node {
            min_value,
            min_index,
            max_value,
            max_index,
        }
    }

    fn set(&mut self, index: usize, value: usize) {
        let mut pos = self.size + index;

        self.tree[pos] = Node {
            min_value: value,
            min_index: index,
            max_value: value,
            max_index: index,
        };

        pos /= 2;

        while pos >= 1 {
            self.tree[pos] = Self::merge(self.tree[pos * 2], self.tree[pos * 2 + 1]);

            if pos == 1 {
                break;
            }

            pos /= 2;
        }
    }

    // [l, r)の範囲を取得
    fn query(&self, mut l: usize, mut r: usize) -> Node {
        l += self.size;
        r += self.size;

        let inf = usize::MAX;

        let mut left_result = Node {
            min_value: inf,
            min_index: 0,
            max_value: 0,
            max_index: 0,
        };

        let mut right_result = left_result;

        while l < r {
            if l % 2 == 1 {
                left_result = Self::merge(left_result, self.tree[l]);
                l += 1;
            }

            if r % 2 == 1 {
                r -= 1;
                right_result = Self::merge(self.tree[r], right_result);
            }

            l /= 2;
            r /= 2;
        }

        Self::merge(left_result, right_result)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [usize; n],
        lr: [(usize, usize); m],
    }

    let mut seg = SegTree::new(&p);

    for (l, r) in lr {
        // 1-indexed → 0-indexed
        let l = l - 1;

        // queryは[l r)なので、Rはそのままで良い
        let result = seg.query(l, r);

        let min_idx = result.min_index;
        let max_idx = result.max_index;

        p.swap(min_idx, max_idx);

        seg.set(min_idx, p[min_idx]);
        seg.set(max_idx, p[max_idx]);
    }

    println!(
        "{}",
        p.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
