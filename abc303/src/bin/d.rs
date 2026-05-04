use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
        s: String,
    }

    let inf = 1_i64 << 60;

    // 初期状態は、caps lockがoff
    let mut dp_off = 0;
    let mut dp_on = inf;

    let ch: Vec<char> = s.chars().collect();

    for c in ch {
        let mut next_off = inf;
        let mut next_on = inf;

        if c == 'a' {
            // 次のcaps lockがoffの場合
            next_off = next_off.min(dp_off + x);
            next_on = next_on.min(dp_off + z + y);

            // 次のcaps lockがonの場合
            next_off = next_off.min(dp_on + z + x);
            next_on = next_on.min(dp_on + y);
        } else {
            // 入力文字が'A'の場合
            // 次のcaps lockがoffの場合
            next_off = next_off.min(dp_off + y);
            next_on = next_on.min(dp_off + z + x);

            // 次のcaps lockがonの場合
            next_off = next_off.min(dp_on + z + y);
            next_on = next_on.min(dp_on + x);
        }
        dp_off = next_off;
        dp_on = next_on;
    }

    println!("{}", dp_off.min(dp_on));
}
