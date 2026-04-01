fn main(){
    let x = 5u32;

    let y = {
        let x_sq = x*x;
        let x_cube = x*x*x;

        x_cube*x_sq+x
    };

    let z = {
        // semicolon suppresses and returns ()
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
