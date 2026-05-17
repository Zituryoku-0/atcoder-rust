use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[path = "../../../lib/lib.rs"]
mod lib;

fn sieve(limit: usize) -> Vec<u64> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=limit {
        if !is_prime[i] {
            continue;
        }

        let mut j = i * 2;
        while j <= limit {
            is_prime[j] = false;
            j += i;
        }
    }

    let mut primes = Vec::new();
    for i in 2..limit {
        if is_prime[i] {
            primes.push(i as u64);
        }
    }

    primes
}

fn main() {
    input! {
        n: u64,
    }

    // N <= 10^12なので、素数は10^6まで用意しておけば十分
    let primes = sieve(1_000_000);

    let mut ans: u64 = 0;
    let len = primes.len();

    for i in 0..len {
        let a = primes[i];

        // a^2が大きすぎる場合は終了
        if a * a > n {
            break;
        }

        for j in i + 1..len {
            let b = primes[j];

            let base = a * a * b;

            // a * a * bがnより大きいなら終了
            if base > n {
                break;
            }

            let limit = n / base;

            // c > bが必要
            // さらにc^2 M= limitを満たすcを探す
            if b * b > limit {
                break;
            }

            // primesの中でp^2 <= limitを満たす範囲の終端を探す
            let upper = primes.partition_point(|&p| p * p <= limit);

            // cはjより後ろ、つまりindexがj+1以上
            if upper > j + 1 {
                ans += (upper - (j + 1)) as u64;
            }
        }
    }

    println!("{}", ans);
}
