use proconio::input;
fn main() {
    input! {
        d: f64
    }
    print!("{}", (d / 2.0) * (d / 2.0) * std::f64::consts::PI);
}
