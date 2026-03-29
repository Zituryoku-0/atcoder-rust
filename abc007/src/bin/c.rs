use proconio::{input, marker::Chars};
use std::collections::VecDeque;
fn main() {
    input! {
        r: usize,
        c: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
        grid: [Chars; r], // これは実質Vec<Vec<char>>である
    }

    // 問題文が1スタートのため、Rustの0スタートに合わせる
    let sy = sy - 1;
    let sx = sx - 1;
    let gy = gy - 1;
    let gx = gx - 1;

    // startからの距離を保持する
    let mut dist = vec![vec![-1_i32; c]; r];

    let mut queue = VecDeque::new();

    // startの位置の距離を0として設定
    dist[sy][sx] = 0;
    queue.push_back((sy, sx));

    // 各方向への移動
    let dirs = [(1_i32, 0_i32), (-1, 0), (0, 1), (0, -1)];

    // queueが存在するまで（何もない場合はNoneが来るので、それで判定）
    while let Some((y, x)) = queue.pop_front() {
        // 各方向への移動をする
        for &(dy, dx) in &dirs {
            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;

            // 壁なら進めない
            if grid[ny][nx] == '#' {
                continue;
            }

            // すでに訪問済みならスキップ
            if dist[ny][nx] != -1 {
                continue;
            }
            // 壁でなく、訪問済みでもないなら、現時点の距離から+1する
            dist[ny][nx] = dist[y][x] + 1;
            queue.push_back((ny, nx));
        }
    }
    println!("{}", dist[gy][gx]);
}
