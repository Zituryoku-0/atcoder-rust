use num::abs;
use num_traits::NumAssign;
use num_traits::NumAssignOps;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }
    let mut nums: Vec<usize> = Vec::new();

    for i in 0..n {
        input! {
            a: usize,
        }
        if let Some(&last_val) = nums.last() {
            let diff = a.abs_diff(last_val);
            if a > last_val && diff > 1 {
                for j in last_val + 1..a {
                    nums.push(j);
                }
            } else if a < last_val && diff > 1 {
                for j in (a + 1..last_val).rev() {
                    nums.push(j);
                }
            }
            nums.push(a);
        } else {
            nums.push(a);
        }
    }

    for i in 0..nums.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", nums[i]);
    }
}
