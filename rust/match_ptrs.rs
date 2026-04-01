fn main(){
    let refer = &4;

    match refer{
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *refer{
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_refer = 3;

    let ref _is_a_refer = 3;

    let value =5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a refer to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
