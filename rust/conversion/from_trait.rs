use std::convert::From;
use std::f32::consts::PI;

#[derive(Debug)]
struct Number {
    value: i32,
}

struct Circle{
    radius: f32,
}


impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<u32> for Circle{
    fn from(value: u32) -> Self{
        Circle{
            radius: value as f32,
        }
    }
}

fn circle_area<T: Into<Circle>>(circle: T) -> f32 {
    let circle = circle.into();
    circle.radius.powi(2) * PI
}

fn main(){
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);


    let circle = Circle::from(3);
    let circle2: Circle = 4.into();
    let circle3 = Into::<Circle>::into(5);
    println!{"The area of circle is {:2}",circle_area(5)};
}
