use proconio::input;
fn main() {
    input! {
        n: usize
    }

    for i in (0..n).rev() {
        print!("{:?}", i + 1);
        if i > 0 {
            print!("{}", ",");
        }
    }
}
