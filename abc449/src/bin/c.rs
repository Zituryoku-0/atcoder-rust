use proconio::input;
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: String
    }

    let mut ans = 0_i64;

    let mut cnt: Vec<i64> = vec![0; 26];

    // a = 97
    let bytes = s.as_bytes();
    // println!("{:?}", bytes);

    for i in 0..bytes.len() {
        if i >= l {
            let idx = (bytes[i - l] - b'a') as usize;
            cnt[idx] += 1;
        }
        if i > r {
            let idx = (bytes[i - r - 1] - b'a') as usize;
            cnt[idx] -= 1;
        }
        let idx = (bytes[i] - b'a') as usize;
        ans += cnt[idx];
    }

    println!("{}", ans);
}
