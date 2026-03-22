use proconio::input;
fn main() {
    input! {
        n: usize,
    }

    // cost[i][j] = 駅iから駅jへの料金（0-indexed）
    let mut cost = vec![vec![0_i64; n]; n];

    for i in 0..n - 1 {
        let len = n - i - 1;
        input! {
            row: [i64; len],
        }
        for k in 0..len {
            let j = i + 1 + k;
            cost[i][j] = row[k];
        }
    }

    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if cost[a][b] + cost[b][c] < cost[a][c] {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No")
}
