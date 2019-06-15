/*
 * RustのMatchフロー制御演算子。
 * CreatedAt: 2019-06-16
 * https://doc.rust-jp.rs/book/second-edition/ch18-03-pattern-syntax.html
 */
fn main() {
    let msg = Message::ChangeColor(255,128,64);
    match msg {
        Message::Quit => println!("Quit!!"),
        Message::Move{x,y} => println!("Move: ({},{})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r,g,b) => println!("ChangeColor: ({},{},{})", r,g,b),
    }
}
enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
    ChangeColor(i32,i32,i32),
}

