#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit; // occupies literally 0 bytes

struct Pair(i32, f32); // mainly used for type safety 

#[derive(Debug)]
struct Point{
    x: f32, 
    y: f32,
}

#[derive(Debug)]
struct Rectangle{
    top_left : Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle)
    -> f32 {
    let Rectangle{ top_left: top_l, bottom_right: bottom_r} = rect;
    let Point{x: x1, y: y1} = top_l;
    let Point{x: x2, y: y2} = bottom_r;
    (x2-x1).abs()*(y2-y1).abs()
}

fn square(point: &Point,sz: f32)
    -> Rectangle {
    Rectangle{
        top_left: Point{  // or we can use *point { This deferences and copies the value to it}
            x: point.x,
            y: point.y,
        },
        bottom_right: Point{
            x: point.x + sz,
            y: point.y + sz,
        },
    }
}

pub fn main() {
   let danny = Person{
        name: String::from("Danny"),
        age: 17,
    };

    println!("{:?}", danny);

    let point_a: Point = Point{
        x: 5.3,
        y: 6.3,
    };

    let point_b: Point = Point{
        x: 10.6,
        y: 12.6,
    };

    println!("point coordinates: ({} {})", point_a.x, point_a.y);

    let bottom_right: Point = Point{x : 10.7, ..point_b};

    println!("The second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point{x: left_edge, y: top_edge }= point_a; 

    let rectangle = Rectangle{
        top_left: Point{x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    
    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("The Area of rectangle: {}", rect_area(&rectangle));

    println!("The square is {:?}", square(&point_a, 4.0));
}
