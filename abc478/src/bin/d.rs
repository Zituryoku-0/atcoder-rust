use im_rc::HashMap;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};

/**
 * 最初0個である配列に対して、L, Rの範囲にXを追加する
 * 同じXは数えない
 *
 * 二次元配列で情報は持てないので、管理方法を工夫する
 *
 *
 *
 */

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut map = HashMap::new();
    for i in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        }
    }
}
