/*
 * RustのMatchフロー制御演算子。
 * CreatedAt: 2019-06-16
 */
fn main() {
    let value = 3;
    match value {
        0 => println!("zero"),
        1 => println!("one"),
        _ => (), // 上記以外のときは`_`で示す。`()`は何もしない。
    }
}

