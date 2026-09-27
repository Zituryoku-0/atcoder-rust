use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

/**
 *
 * 解説
 * 1.まずは、Sの中でTと一致し、かつS[i] == S[0]のものをVecに追加する
 * 2.その後、二分探索でl.rの範囲を満たすものを特定する
 */

fn lower_bound(a: &[usize], x: usize) -> usize {
    let mut left = 0;
    let mut right = a.len();

    while left < right {
        let mid = (left + right) / 2;

        if a[mid] < x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    input! {
    q: usize,
    s: Chars,
    t: Chars,
    }

    let n = s.len();
    let m = t.len();

    // TがSのどの位置から始まるか
    // 0-indexedで保持する
    let mut positions = Vec::new();

    if n >= m {
        for i in 0..=n - m {
            let mut ok = true;

            for j in 0..m {
                if s[i + j] != t[j] {
                    ok = false;
                    break;
                }
            }

            if ok {
                positions.push(i);
            }
        }
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        // 問題文は1-indexedなので0-indexedに変換
        let l = l - 1;
        let r = r - 1;

        // 区間の長さ自体がTより短ければ絶対に含まれない
        if r - l + 1 < m {
            println!("No");
            continue;
        }

        /**
         * Tの開始位置xが
         * l <= x
         * x + m - 1 <= r
         * を満たせばよい
         *
         * つまり
         * 1 <= x <= r - m + 1
         * を満たす開始位置があるか調べる
         */
        let max_start = r - m + 1;

        // positionsの中でl以上となる最初の位置
        let index = lower_bound(&positions, l);

        if index < positions.len() && positions[index] <= max_start {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
