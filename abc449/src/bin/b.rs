use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        q: usize,
        query: [[usize; 2]; q]

    }
    let mut _choco: Vec<Vec<i32>> = vec![vec![1; w]; h];

    // print!("{:?}", _choco);

    for i in 0..q {
        if query[i][0] == 1 {
            // print!("クエリ1での対象数字：{}", query[i][1]);
            // タイプ1
            let res1 = w * query[i][1];
            h -= query[i][1]; // クエリの行分消す
            println!("{}", res1);
        } else {
            // タイプ2
            // print!("クエリ2での対象数字：{}", query[i][1]);
            let res2 = h * query[i][1];
            w -= query[i][1]; // クエリの列分消す
            println!("{}", res2);
        }
    }
}
