/*
 * RustのMatchフロー制御演算子。
 * CreatedAt: 2019-06-16
 */
fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter));
    
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => { println!("Lucky penny !!"); 1 },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
