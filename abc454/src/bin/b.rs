use im_rc::HashSet;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut data: Vec<usize> = vec![0usize; m];
    let mut set: HashSet<usize> = HashSet::new();

    for i in 0..n {
        input! {
            f: usize,
        }
        let f = f - 1;
        data[f] += 1;
        set.insert(f);
    }

    let mut ok = true;
    for i in 0..m {
        if data[i] > 1 {
            ok = false;
            break;
        }
    }
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }

    if set.len() >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
