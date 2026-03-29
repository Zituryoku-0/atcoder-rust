use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut departments: Vec<(usize, usize)> = vec![(0, 0); n];

    for i in 0..n {
        input! {
            department: (usize, usize),
        }
        departments[i] = department;
    }
    // println!("{:?}", departments);

    let mut before: Vec<i32> = vec![0; m];
    let mut after: Vec<i32> = vec![0; m];

    for i in 0..n {
        before[departments[i].0 - 1] += 1;
        after[departments[i].1 - 1] += 1;
    }

    // println!("beforeの値：{:?}", before);
    // println!("afterの値：{:?}", after);

    for i in 0..m {
        println!("{:?}", after[i] - before[i]);
    }
}
