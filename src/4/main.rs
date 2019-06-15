/*
 * RustのMatchフロー制御演算子。
 * CreatedAt: 2019-06-16
 * https://doc.rust-jp.rs/book/second-edition/ch18-03-pattern-syntax.html
 */
fn main() {
    let value = 3;
    match value {
        0 => println!("zero"),
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4 ... 9 => println!("four ... nine"),
        _ => (), // 上記以外のときは`_`で示す。`()`は何もしない。
    }
}

