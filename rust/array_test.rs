use std::mem; 


pub fn main() {
    let xs: [i32; 5] = [1,2,3,4,5];

    let ys: [i32; 500] = [0; 500];

    println!("first element of array: {}", xs[0]);
    println!("Second element of array: {}", xs[1]);

    println!("The number of elements in the array: {}", xs.len());

    println!("Array occupies {} bytes", mem::size_of_val(&xs));
}
