use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[path = "../../../lib/lib.rs"]
mod lib;

fn main() {
    // 解けなかった問題
    // この問題は、各頂点の距離と、円の半径を使って求める

    // ポイントとしては、接していないパターンを考える
    // ・円が離れすぎているパターン（r1 + r2 < Dであるパターン）
    // ・円が内包しているパターン(|r1 - r2| > Dであるパターン) ※この考えを忘れがち
    // これ以外はYesである（|r1-r2| <= D <= r1 + r2）である
    // 注意点
    // 少数による誤差が出る可能性があるので、できれば整数で計算できるようにしたい
    // これを実現するために、2乗してあげると良い
    // これ以外はYesである（|r1-r2|^2 <= D^2 <= (r1 + r2)^2）である
    // D = √(x1 - x2)^2 + (y1 - y2)^2である

    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            x1: i128,
            y1: i128,
            r1: i128,
            x2: i128,
            y2: i128,
            r2: i128,
        }

        let x_diff = x1 - x2;
        let y_diff = y1 - y2;

        // 中心間距離の2乗
        let distance = x_diff * x_diff + y_diff * y_diff;

        // 半径の差と合計
        let radius_diff = (r1 - r2).abs();
        let radius_sum = r1 + r2;

        if radius_diff * radius_diff <= distance && distance <= radius_sum * radius_sum {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
