fn temp_fn(i: &u32){
    println!("The memory address of the static int in fn: {:p}", &i);
    println!("The memory address of the static int in fn: {:p}", i);
    println!("The memory address of the static int in fn: {}", *i);
}


fn main() {
    static INTEGER : u32 = 1;
    let boolean = true;
    let unit = ();

    let copied = integer;

    println!("The memory address of static int in main: {:p}", &integer);
    temp_fn(&integer);
    
    // println!("An int: {}", integer);
    // println!("A bool: {}", boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused = 3u32;

    let noisy_unused = 2u32;
}


