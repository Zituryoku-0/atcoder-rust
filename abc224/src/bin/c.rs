use proconio::input;
fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n]
    }

    let mut ans = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if (points[k].0 - points[i].0) * (points[j].1 - points[i].1)
                    != (points[k].1 - points[i].1) * (points[j].0 - points[i].0)
                {
                    ans += 1;
                }
            }
        }
    }

    print!("{}", ans);
}
