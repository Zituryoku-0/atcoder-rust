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

    // dp_off = 今まで打ち終わってCapsLockがOFFの最小コスト
    // dp_on = 今まで打ち終わってCapsLockがONの最小コスト
    let mut dp_off = 0_i64;
    let mut dp_on = inf;

    for ch in s.chars() {
        let mut next_off = inf;
        let mut next_on = inf;

        if ch == 'a' {
            // 現在OFF -> そのままaを打つ
            next_off = next_off.min(dp_off + x);
            // 現在OFF -> CapsLockをONにしてShift+aを打つ
            next_on = next_on.min(dp_off + z + y);

            // 現在ON -> そのままShift+aでaを打つ
            next_on = next_on.min(dp_on + y);
            // 現在ON -> CaplsLockをOFFにしてaを打つ
            next_off = next_off.min(dp_on + z + x);
        } else {
            // ch == 'A'

            // 現在OFF -> そのままShift+aでAを打つ
            next_off = next_off.min(dp_off + y);
            // 現在OFF -> CapsLockをONにしてaを打つ
            next_on = next_on.min(dp_off + z + x);

            // 現在ON -> そのままaを打つ
            next_on = next_on.min(dp_on + x);
            // 現在ON -> CapsLockをOFFにしてShift+aでAを打つ
            next_off = next_off.min(dp_on + z + y);
        }

        dp_off = next_off;
        dp_on = next_on;
    }

    println!("{}", dp_off.min(dp_on));
}
