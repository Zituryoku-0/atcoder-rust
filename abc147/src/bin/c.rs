use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;
use std::usize;

/**
* 解説
* 今回のポイントは、正直者と仮定した人の証言のみを全部チェックすることにある
* 不親切な人の証言は間違っている場合もあるため、その人は証言のチェックは不要である
*
* 1.bit全探索の流れ
* for bit in 0..(1usize << n) {}
* で全パターンをn^2で試すことができる
* そこから、0→不親切、1→正直者と仮定して
* for i in 0..n {
*   if ((bit >> i) & 1) == 1 {}
* }
* とすることで、その時のbitが1の人は正直者として処理を進めることができる
* ここで、(bit >> i) & 1でその時のiのbit1つを取得することができる
*
* 2.証言のチェック方法について
* 正直者と仮定した人iの証言が、仮定しているものと一致している必要がある
* 例えば、人2は正直者である、すなわち
* x = 2, y = 1の場合
* この時、現在のbitで人2が正直者であればOKである
* ((bit >> (x - 1)) & 1) == 1であれば良い
*
* また、もし人2を不親切な人と証言している場合は
* ((bit >> (x - 1)) & 1) == 0である必要がある
*
* すなわち
* ((bit >> (x - 1)) & 1) == yであれば、その人が不親切で一致、もしくは正直者で一致することを表現することができる
* 逆に((bit >> (x - 1)) & 1) != yであれば矛盾することになる
*
* 3.一通り、証言をチェックして、一つも矛盾がなかった場合は、その時のbitで1が立っているものをカウントする
* その中で最大のものが答えになる
* bit.count_ones() as usizeでbitに1が立っている数をカウントすることができる
* ans = ans.max(bit.count_ones() as usize);となる
*/

fn muzyun(shougen: &[(usize, usize)], current_bit: usize) -> bool {
    for &(x, y) in shougen {
        let actual = (current_bit >> (x - 1)) & 1;

        if actual != y {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
    }

    let mut shougen = vec![Vec::with_capacity(n); n];

    for i in 0..n {
        input! {
            _a: usize,
        }

        for _ in 0.._a {
            input! {
                x: usize,
                y: usize,
            }
            shougen[i].push((x, y));
        }
    }

    let mut ans = 0;
    // bit全探索する
    for bit in 0..(1usize << n) {
        let mut ok = true;

        // 人iを確認
        for i in 0..n {
            // 人iを正直者と仮定していないなら、
            // 証言内容を確認する必要はない
            if ((bit >> i) & 1) == 0 {
                continue;
            }

            // 正直者と仮定した人の証言を全チェック
            let check = muzyun(&shougen[i], bit);
            if !check {
                ok = false;
                break;
            }
        }
        if ok {
            ans = ans.max(bit.count_ones() as usize);
        }
    }

    println!("{}", ans);
}
