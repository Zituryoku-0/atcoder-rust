use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 * 解説
 * この問題は、最初は全て同じ長さLのお菓子だったという点から、候補となるLをかなり絞り込める問題である
 * 候補は高々2つしかない
 * 　・各お菓子が、そのまま残る
 * 　・2本に割れて合計がLになる
 *
 * 重要な観察
 * まずAを照準にソートして考える
 * パターン1：すべてのお菓子が割れた
 * ・最終的な本数はすべて偶数である必要がある
 * ・かつ、Ai + Aj = Lに全てなる必要がある
 * ・ここで、一番小さいA[0]と一番大きいA[n-1]はペアになるしかないので、
 *  A[0] + A[n-1], A[1] + A[n-2]、、、といった組み合わせで全て等しい必要がある
 * 例：
 * A = [5, 5, 10, 10]ならば
 * 5 + 10 = 15で全て15になり、これは可能である
 *
 * パターン2：割れなかったお菓子が存在する
 * L→Lとしてそのまま残ったものが1本以上存在する場合
 * 割れてできたお菓子は(1以上, L-1以下)という組み合わせになる
 * ソート後の場合は、a[n-1]にあることになる
 *
 * L = max(A)の時は何を確認するか？
 * 例えば A = [5, 5, 10, 10]でL = 10の場合
 * 後ろの10は割れなかったものになる
 * 割れなかったものを取り除くと[5, 5]なので
 * ここから5 + 5 = 10 = LであればOK
 * つまり、max(A)と等しい要素は全部そのまま残ったものとして除外し、
 * L未満の要素だけをペアにして、その和 = Lになるかを確認できれば良い
 *
 * ここで、候補となるLは
 * ①max(A)　← 割れなかったもの
 * ②min(A) + max(A) ← 割れた中で、最小と最大の和
 *
 *
 */

fn can_make(a: &[i64], l: i64) -> bool {
    let mut left = 0usize;
    let mut right = a.len();

    // Lそのものは割れずに残ったものとみなせる
    while right > 0 && a[right - 1] == l {
        right -= 1;
    }

    // 残った本数が奇数ならペアを作れない
    if right % 2 == 1 {
        return false;
    }

    if right == 0 {
        return true;
    }

    let mut r = right - 1;

    while left < r {
        // 最小と最大の和がlと等しくないならfalse
        if a[left] + a[r] != l {
            return false;
        }

        left += 1;
        r -= 1;
    }

    true
}

fn main() {
    input! {
        n: usize,
        mut a: [i64;n ],
    }

    a.sort_unstable();

    let mut ans = Vec::new();

    // 候補1：割れなかったものが存在する場合
    let l1 = a[n - 1];

    if can_make(&a, l1) {
        ans.push(l1);
    }

    // 候補2：全部割れた場合
    if n % 2 == 0 {
        let l2 = a[0] + a[n - 1];
        if l2 != l1 && can_make(&a, l2) {
            ans.push(l2);
        }
    }

    ans.sort_unstable();

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
