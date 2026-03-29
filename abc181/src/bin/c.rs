use proconio::input;
fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n]
    }

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                let (x3, y3) = points[k];

                // 3点が一直線上にあるか判定
                if (y2 - y1) * (x3 - x1) == (x2 - x1) * (y3 - y1) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
