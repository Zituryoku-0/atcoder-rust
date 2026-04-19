use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn normalize(s: &str) -> Vec<u8> {
    let mut stack: Vec<u8> = Vec::with_capacity(s.len());

    for &ch in s.as_bytes() {
        stack.push(ch);

        if stack.len() >= 4 {
            let n = stack.len();
            if stack[n - 4] == b'('
                && stack[n - 3] == b'x'
                && stack[n - 2] == b'x'
                && stack[n - 1] == b')'
            {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(b'x');
                stack.push(b'x');
            }
        }
    }
    stack
}

fn main() {
    input! {
        t: usize,
    }
    // let mut answers = Vec::with_capacity(t);

    for _ in 0..t {
        input! {
            a: String,
            b: String,
        }

        let na = normalize(&a);
        let nb = normalize(&b);

        if na == nb {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
