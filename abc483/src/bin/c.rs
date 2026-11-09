use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        touhyou: [(usize, usize); m],
    }

    let mut base = vec![0usize; n];
    let mut second_choices = vec![Vec::new(); n];
    for (a, b) in touhyou {
        let (a, b) = (a - 1, b - 1);
        base[a] += 1;
        second_choices[a].push(b);
    }

    let mut first: Option<(usize, Reverse<usize>)> = None;
    let mut second: Option<(usize, Reverse<usize>)> = None;
    for j in 0..n {
        let kouhosha = (base[j], Reverse(j));
        if first.is_none_or(|x| kouhosha > x) {
            second = first;
            first = Some(kouhosha);
        } else if second.is_none_or(|x| kouhosha > x) {
            second = Some(kouhosha);
        }
    }

    let mut added = vec![0usize; n];
    let (_, Reverse(first_index)) = first.unwrap();
    for i in 0..n {
        for &b in &second_choices[i] {
            added[b] += 1;
        }

        // 票数が変わらない候補の最大値。
        let notchange = if first_index == i { second } else { first };
        let mut best = notchange.unwrap_or((0, Reverse(i)));

        // i への票を引き、第二候補への票を加える。
        for &b in &second_choices[i] {
            if b != i {
                best = best.max((base[b] + added[b], Reverse(b)));
            }
        }

        for &b in &second_choices[i] {
            added[b] = 0;
        }
        println!("{}", best.1 .0 + 1);
    }
}
