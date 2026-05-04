use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash;
use std::hash::Hash;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: i32,
        k: usize,
        s: String,
    }

    let mut item = HashSet::new();
    let mut h = h;
    for _ in 0..m {
        input! {
            x: i32,
            y: i32,
        }
        item.insert((x, y));
    }

    let mv: Vec<char> = s.chars().collect();

    let mut grid_x: i32 = 0;
    let mut grid_y: i32 = 0;
    let mut count = 0;

    for i in 0..n {
        match mv[i] {
            'R' => grid_x += 1,
            'L' => grid_x -= 1,
            'U' => grid_y += 1,
            _ => grid_y -= 1,
        }
        h -= 1;
        // 体力が負の数になったら終わり
        if h < 0 {
            println!("No");
            return;
        }
        if h < k as i32 && item.remove(&(grid_x, grid_y)) {
            h = k as i32;
        }
    }

    println!("Yes");
}
