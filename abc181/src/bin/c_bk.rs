use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        n: u64,
    }

    // 入力を受け取る
    let mut plane: Vec<Vec<u64>> = Vec::new();
    for _i in 0..n {
        input! {
            row: [u64; 2],
        }
        // println!("row:{:?}", row);
        plane.push(row);
    }

    // println!("plane:{:?}", plane);
    // 処理
    let mut map_x = HashMap::new();
    let mut map_y = HashMap::new();

    for i in 0..plane.len() {
        let count_x = map_x.entry(plane[i][0]).or_insert(0);
        *count_x += 1;
        let count_y = map_y.entry(plane[i][1]).or_insert(0);
        *count_y += 1;
    }
    println!("map_x:{:?}", map_x);
    println!("map_y:{:?}", map_y);

    let set_x: HashSet<u64> = map_x.values().copied().collect();
    let set_y: HashSet<u64> = map_y.values().copied().collect();

    println!("set_x:{:?}", set_x);
    println!("set_y:{:?}", set_y);

    for x in set_x.iter() {
        if *x >= 3 {
            println!("Yes");
            return;
        }
    }

    for y in set_y.iter() {
        if *y > 3 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
