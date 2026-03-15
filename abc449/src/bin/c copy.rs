use proconio::input;
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: String
    }

    let mut res = 0;

    // char配列でもつ
    let mut chars: Vec<char> = s.chars().collect();
    chars.insert(0, ' ');

    let min_range = r - l;
    // let mix_range = r;

    // println!("配列の長さ：{}", chars.len());

    for cur in 1..chars.len() - min_range {
        // while i in cur..cur + r {
        let mut i = cur;
        let mut j = cur + min_range;
        let mut target = j + min_range + 1;
        while j < target && j < chars.len() && j - i <= r {
            if chars[i] == chars[j] {
                // println!("iの値：{}", i);
                // println!("jの値：{}", j);
                res += 1;
            }
            j += 1;
        }
    }
    println!("{}", res);
}
