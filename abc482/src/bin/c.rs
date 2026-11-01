use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 * 解説
 * myansでは、HashMap→Vec→sort→累積和としているが、
 * HashMapも累積話も必須ではない
 *
 * なぜなら、各コマのR_iだけを全部配列に入れてソートすれば
 * [u, d]に入るR_iの個数は二分探索のみで直接求めることができるからである
 *
 * 例えば、
 * rows = [1, 2, 2, 4, 5]と持っておけば
 * let start = rows.partition_point(|&x| x < u);
 * let end = rows.partition_point(|&x| x <= d);
 * println!("{}", end - start);
 * だけで済むからである
 *
 * つまり、この問題の本質は
 * 1.座標をソートする
 * 2.区間[L, R]に入る要素数を二分探索で求める
 * だけである
 */

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        q: usize,
    }

    let mut rows = Vec::with_capacity(n);
    let mut cols = Vec::with_capacity(n);

    for _ in 0..n {
        input! {
            r: usize,
            c: usize,
        }

        rows.push(r);
        cols.push(c);
    }
    rows.sort_unstable();
    cols.sort_unstable();

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                u: usize,
                d: usize,
            }

            /**
             * なぜこれだけで求められるのか
             *
             * 例えばコマの行番号が
             * 1 2 2 4 5の場合
             * ソート後のrowsは
             * rows = [1, 2, 2, 4, 5]となる
             * ここで、2行目から4行目にある駒はいくつ？を考えるとする
             * 欲しいのは2 <= row <= 4を満たす要素数である
             * 対象は[2, 2, 4]の3つである
             *
             * 1.左端を2分探索する
             * let left = rows.partition_point(|&x| x < u);
             * で今回u = 2なので具体的には
             * rows.partition_point(|&x| x < 2);となっている
             *
             * partiton_pointは、「条件がtrueでなくなる最初の位置」を返すため
             * rows = [1, 2, 2, 4, 5]の場合、最初に x < 2を満たさなくなるインデックス1を返す
             * すなわち2以上の要素が始まる位置を表す
             *
             * 2.右端を2分探索する
             * let right = rows.partition_point(|&x| x <= d);
             * で今回d = 4なので具体的には
             * rows.partition_point(|&x| x <= 4);となっている
             *
             * rows = [1, 2, 2, 4, 5]のため、最初にx <= 4を満たさなくなるインデックス4を返している
             * すなわち4より大きい（4以下の要素が終わった直後の位置）を表す
             *
             * 3.right - leftで個数になる
             * ここまでで、
             * left = 1, right = 4が求めれている
             * つまり、対象となる添字は
             * [1, 4)である（インデックス1を含み、4は含まない）
             * Rustの区間で考えると
             * rows[1..4]（for文と同じ考え方をすると、1,2,3）まで
             * 要素数は4 - 1 = 3となる
             *
             *
             *
             */
            let left = rows.partition_point(|&x| x < u);
            let right = rows.partition_point(|&x| x <= d);
            println!("{}", right - left);
        } else {
            input! {
                l: usize,
                r: usize,
            }

            let left = cols.partition_point(|&x| x < l);
            let right = cols.partition_point(|&x| x <= r);

            println!("{}", right - left);
        }
    }
}
