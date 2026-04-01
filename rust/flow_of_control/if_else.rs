fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is neg", n);
    } else if n > 0 {
        print!("{} is pos", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small num, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big num, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}
