use proconio::input;
fn main() {
    input! {
        n: i32,
        arrays: [i32; n]
        multi_arrays: [[i32; n]; n]
    }
    print!("{}", n);
    print!("{:?}", arrays);
    print!("{:?}", multi_arrays);
}
