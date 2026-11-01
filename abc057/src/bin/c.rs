use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 * 解説
 * この問題は、「約数を√Nまで調べれば良い」という典型問題である
 * N = A * Bを満たす正の整数A,Bのうち、'Aの桁数'と'Bの桁数'の大きい方を最小化する
 *
 * ポイント1
 * 「約数はペアで現れる」という点である
 * 例えば、N = 36なら、
 * 1 * 36
 * 2 * 18
 * 3 * 12
 * 4 * 9
 * 6 * 6
 * である（このあとは、9 * 4 、、、と前半と逆のものが出てくるだけである）
 *
 * つまり、A <= Bだけ調べれば十分である
 *
 * 従って、for a in 1..=sqrt(n)でループさせれば良い
 * もしくは while a * a <= nでも良い
 *
 *
 * ポイント2
 * 約数だったら、Bは自動的に決まる点である
 * aが約数かどうかは、 if n % a == 0で判定できる
 * 約数であれば「let b = n / a」である
 *
 * 例えば、
 * N = 10000
 * a = 100
 * の時
 *
 * 10000 % 100 = 0なので
 * b = 10000 / 100 = 100となる
 *
 * ポイント3
 * 数字の桁数を簡単に求めることができる
 * Rustでは整数を文字列に変換して
 * x.to_string().len()とすれば数字が文字列に変換される
 *
 * そのため、AとBの文字列の大きい方を比較するのは
 * let digits_a = a.to_string().len();
 * let digits_b = b.to_string().len();
 * let f = digits_a.max(digits_b);
 * で求めることができる
 *
 *
 *
 */

fn main() {
    input! {
        n: usize,
    }

    let mut a = 1;
    let mut b = 0;
    let mut ans = usize::MAX;
    // a^2がn以下の間ループさせる
    while a * a <= n {
        // aがnの約数である場合
        if n % a == 0 {
            b = n / a;

            // AとBの桁数を求める
            let digits_a = a.to_string().len();
            let digits_b = b.to_string().len();

            // 桁数が大きい方をその時の答えとする
            let f = digits_a.max(digits_b);

            ans = ans.min(f);
        }

        a += 1;
    }

    println!("{}", ans);
}
