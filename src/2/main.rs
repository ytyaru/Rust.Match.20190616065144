/*
 * RustのMatchフロー制御演算子。
 * CreatedAt: 2019-06-16
 */
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Noneパターン未実装ならコンパイルエラーになる: error[E0004]: non-exhaustive patterns: `None` not covered
        Some(i) => Some(i+1),
    }
}

