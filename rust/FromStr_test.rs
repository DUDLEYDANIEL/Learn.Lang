use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: f32, 
}

impl FromStr for Circle{
    type Err = ParseFloatError;
    fn from_str(s: &str)
        -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle{radius: num}),
            Err(e)  => Err(e),
        }
    }
}

fn main() {
    let radius = "      3.0";
    let circle:Circle = radius.parse().unwrap();
    println!("{:?}", circle);
}
