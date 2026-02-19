#![allow(dead_code)]

enum Number {
    Zero,
    One, 
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    use Number::*;
    use Color::*;
    println!("zero is {}", Zero as i32);
    println!("one is {}", One as i32);
    println!("roses are #{:06x}", Red as u32);
}
