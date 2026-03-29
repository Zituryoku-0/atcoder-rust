use proconio::input;
fn main() {
    input! {
        n: usize,
        l: [i64; n],
    }
    let mut nums: Vec<i64> = l;
    nums.sort();

    let mut ans = 0;
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                if nums[i] != nums[j] && nums[j] != nums[k] && nums[k] + nums[j] > nums[i] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
