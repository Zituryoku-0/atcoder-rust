// 切り上げ割り算をする
pub fn divceil(a: i128, b: i128) -> i128 {
    (a + b - 1) / b
}
