/*
 * RustのMatchフロー制御演算子。
 * CreatedAt: 2019-06-16
 * https://doc.rust-jp.rs/book/second-edition/ch18-03-pattern-syntax.html
 */
fn main() {
    // Point構造体の各値を個別の変数に分配する
    let p1 = Point{ x:0, y:1 };
    let Point{ x:a, y:b } = p1;
    assert_eq!(0, a);
    assert_eq!(1, b);
    // 略記
    let p2 = Point{ x:1, y:0 };
    let Point{ x, y } = p2;
    assert_eq!(1, x);
    assert_eq!(0, y);
    // Match
    let p3 = Point{ x:1, y:1 };
    match p3 {
        Point {x, y:0} => println!("on the x axis at {}", x),
        Point {x:0, y} => println!("on the y axis at {}", y),
        Point {x, y} => println!("on neither axis: ({},{})", x,y),
    }
}
struct Point {
    x: i32,
    y: i32,
}
