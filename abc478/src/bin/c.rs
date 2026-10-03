use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 *
 * 解説
 * 最終的に作りたい配列をソートしておく
 * すると、元の配列とソートの違っている場所は、必ず今回ソートする長さKの区間の中に全部入っていなければならない
 * 例えば
 * A      = 1 4 1 4 2 1 3 5 6
 * sorted = 1 1 1 2 3 4 4 5 6
 *
 * 違う部分は
 *       ┌─────────┐
 * A      1 4 1 4 2 1 3 5 6
 * sorted 1 1 1 2 3 4 4 5 6
 *         ^         ^
 *         L         R
 * ここを全て含む長さKの区間が存在するなら、その区間をソートすれば良い
 * つまり、
 * 最後の不一致位置 - 最初の不一致位置 + 1 <= K
 * ならYesである
 *
 *
 *
 */

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut sorted = a.clone();
    /**
     * Rust sortとsort_unstableの違い
     * 同じ数字の要素の順番を保持する：sort
     * 保持しない：sort_unstable
     *
     * もちろんsort_stableの方が高速である
     * 今回の場合、同じ数字内での順番の保持は不要なため、
     * ここではsort_unstableを使う方が効果的である
     */
    sorted.sort_unstable();

    let mut left = None;
    let mut right = None;

    for i in 0..n {
        if a[i] != sorted[i] {
            if left.is_none() {
                left = Some(i);
            }
            right = Some(i);
        }
    }

    // 最初から昇順なら、どこをソートしても照準のまま
    let Some(left) = left else {
        println!("Yes");
        return;
    };

    let right = right.unwrap();

    if right - left + 1 <= k {
        println!("Yes");
    } else {
        println!("No");
    }
}
