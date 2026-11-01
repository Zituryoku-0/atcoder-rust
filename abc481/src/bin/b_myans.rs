use ordered_float::FloatIsNan;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::SeekFrom::Current;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 * 移動先もしくは、移動中にkが含まれれば良い
 * A→Bの幅を見てその中にkがあればカウントする
 *
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut current_goal = 1000;
    let mut check = false;
    for i in 1..n {
        let mut current_nums = Vec::new();
        if a[i] == a[i - 1] && a[i] == k {
            ans += 1;
            check = true;
            continue;
        }
        if a[i] > a[i - 1] {
            for j in a[i - 1]..=a[i] {
                if current_goal == j {
                    continue;
                }
                current_nums.push(j);
            }
            // println!("{:?}", current_nums);

            for num in current_nums {
                if num == k {
                    ans += 1;
                }
            }
            check = false;
            current_goal = a[i];
        } else {
            for j in (a[i]..=a[i - 1]).rev() {
                if current_goal == j {
                    continue;
                }
                current_nums.push(j);
            }

            // println!("{:?}", current_nums);
            for num in current_nums {
                if num == k {
                    ans += 1;
                }
            }
            check = false;

            current_goal = a[i];
        }
    }

    if check {
        println!("{}", ans + 1);
    } else {
        println!("{}", ans);
    }
}
