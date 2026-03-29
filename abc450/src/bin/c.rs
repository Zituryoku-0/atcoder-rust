use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [String; h],
    }

    let grid: Vec<Vec<char>> = s.into_iter().map(|row| row.chars().collect()).collect();
    let mut visited = vec![vec![false; w]; h];

    let dirs = [(-1_i32, 0_i32), (1, 0), (0, -1), (0, 1)];
    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            // 白マスで、まだ未訪問の時だけ新しい連結成分の探索を始める
            if grid[i][j] != '.' || visited[i][j] {
                continue;
            }

            let mut que = VecDeque::new();
            que.push_back((i, j));
            visited[i][j] = true;

            // この連結成分が外周に触れているか
            let mut touch_border = false;

            while let Some((x, y)) = que.pop_front() {
                // 外周判定
                if x == 0 || x == h - 1 || y == 0 || y == w - 1 {
                    touch_border = true;
                }
                for &(dx, dy) in &dirs {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                        continue;
                    }

                    let nx = nx as usize;
                    let ny = ny as usize;

                    if visited[nx][ny] {
                        continue;
                    }

                    if grid[nx][ny] != '.' {
                        continue;
                    }

                    visited[nx][ny] = true;
                    que.push_back((nx, ny));
                }
            }
            if !touch_border {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
