/*
 * RustのMatchフロー制御演算子。
 * CreatedAt: 2019-06-16
 * https://doc.rust-jp.rs/book/second-edition/ch18-03-pattern-syntax.html
 * マッチガード。
 */
fn main() {
    let value = Some(-5);
    match value {
        Some(0) => println!("zero"),
        Some(1) | Some(2) => println!("one or two"),
        Some(i) if i < 0 => println!("minus {}", i),
        Some(i) if i >= 0 => println!("plus {}", i),
        None => (),
        _ => (),
    }
}

