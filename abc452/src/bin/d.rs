use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    let n = s_chars.len();
    let inf = n; // 「存在しない」を表す

    // next_pos[i][c] = 位置i以上で、文字cが最初に現れる位置
    let mut next_pos = vec![[inf; 26]; n + 1];

    // next_pos[n]は全部infのまま
    for i in (0..n).rev() {
        next_pos[i] = next_pos[i + 1];
        let idx = (s_chars[i] as u8 - b'a') as usize;
        next_pos[i][idx] = i;
    }

    let mut ans: i64 = 0;

    for l in 0..n {
        let mut pos = l;
        let mut ok = true;

        for &ch in &t_chars {
            // 文字をusizeにしてインデックス化
            let idx = (ch as u8 - b'a') as usize;
            if pos > n || next_pos[pos][idx] == inf {
                ok = false;
                break;
            }
            pos = next_pos[pos][idx] + 1;
        }

        if ok {
            // Tを部分列として含む最小終点rはpos-1
            let r = pos - 1;
            ans += (r - l) as i64;
        } else {
            // どこまで伸ばしてもTを作れない
            ans += (n - l) as i64;
        }
    }
    println!("{}", ans);
}
