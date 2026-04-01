use std::convert::TryFrom;
use std::convert::TryInto;

struct Circle {
    radius: f32
}

impl TryFrom<f32> for Circle {
    type Error = &'static str;

    fn try_from(value:f32)
        -> Result<Self, Self::Error> {
        if value >= 0.0 {
            Ok(Circle{ radius: value })
        } else{
            Err("Circle cannot have a negative value")
        }
    }
}

fn main(){
    let int: f32 = -3.0;
    let circle: Circle = int.try_into().expect("Conversion failed");
    print!("Type converted successfully");
}
