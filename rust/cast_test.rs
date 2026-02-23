#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;
    
    // no implicit type conversion
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;
        
    // cannot convert float to char
    // let character = decimal as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is: {}", 1000 as u16);
    println!("1000 as u8 is: {}", 1000 as u8);

    println!("-1 as a u8 is: {}", (-1i8) as u8);


}
