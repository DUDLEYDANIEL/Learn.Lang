type NanoSec = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let nanasecond: NanoSec = 5;
    let inches: Inch = 2;

    println!("{} nanasecond + {} inches = {} units", nanasecond, inches, nanasecond+inches);
}
