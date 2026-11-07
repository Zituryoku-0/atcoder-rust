use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufWriter, Write};
use std::println;

/**
 * 解説
 * この問題は、一見「どのドミノをどの順番で選ぶか」を探索する問題に見えるが、
 * ポイントは「今倒せる中でできるだけ大きなドミノを選ぶ」貪欲法である
 *
 * 1.まず問題を単純化する
 * ドミノの番号そのものは、基本的には重要ではない
 * 重要なのは
 * S1 → ? → ? → ... → SN
 * という大きさの列を作って
 * 次の大きさ <= 現在の大きさ * 2
 * を全て満たせるかどうかである
 *
 * 2.どのドミノを次に選べば良いか？
 * 現在
 * current = 10だとする
 * この場合、次に選べるのは
 * S[i]<= 20のドミノである
 *
 * 候補が[11, 15, 18, 20]
 * の場合、どれを選ぶべきか？
 * 答えは20である
 * なぜなら、次のドミノが大きいほど、その次に倒せる範囲も広くなるためである
 * 11を選ぶ → 次は22以下
 * 20を選ぶ → 次は40以下
 * ため、20の方が有利である
 *
 * つまり、現在倒せるドミノの中から、最大のものを選ぶという貪欲法が成立する
 *
 * 3.まずSNに直接届くかを確認する
 * 現在の大きさをcurrentとしたときに、
 * もし
 * current * 2 >= s[n-1]であれば、最後のドミノNを直接倒すことができるため、その時点で終了である
 *
 * 4.直接届かない場合
 * 例えば
 * current = 3
 * SN = 20の場合
 *
 * 3 * 2 = 6 < 20のため、直接は届かない
 * そこで、6以下のドミノを探す
 * 候補が[2, 4, 5, 6, 10, 15]の場合は、
 * 6を選ぶのが最善である
 * そこからcurrent = 6の場合は、今度は12以下の最大を探す
 *
 * つまり、
 * current = 3
 * ↓
 * 6以下で最大 → 6
 *
 * current = 6
 * ↓
 * 12以下で最大 → 10
 *
 * current = 10
 * ↓
 * 2 * 10 >= 20
 * ↓
 * SNへ
 * となる
 *
 * 5.進めなくなるケース
 * 例えば
 * S1 = 2
 * SN = 100
 *
 * 途中のドミノ[1, 2, 10, 20]の場合
 * 最初はcurrent = 2なので、4以下しか選べない
 * しかし候補が[1, 2]のみである
 * そのため、ずっと大きくなることはない
 * したがって
 * 現在より大きく、かつ2 * current以下ののドミノが存在しなければ-1になる
 *
 *
 *
 *
 */

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: [u64; n],
        }
        let start = s[0];
        let goal = s[n - 1];

        // ドミノ1とN以外を取り出す
        let mut middle = s[1..n - 1].to_vec();

        middle.sort_unstable();

        let mut current = start;

        // 最初にドミノ1を置いている
        let mut ans = 1usize;

        loop {
            // 現在のドミノからNを直接倒せる
            if current * 2 >= goal {
                ans += 1;
                println!("{}", ans);
                break;
            }

            // current * 2以下の最大のドミノを探す
            let pos = middle.partition_point(|&x| x <= current * 2);

            // 候補が1つもない
            if pos == 0 {
                println!("-1");
                break;
            }

            let next = middle[pos - 1];

            // currentより大きくならないなら、
            // これ以上先に進めない
            if next <= current {
                println!("-1");
                break;
            }

            current = next;
            ans += 1;
        }
    }
}
