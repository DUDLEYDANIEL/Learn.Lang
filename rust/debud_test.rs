#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep<'a>(&'a Structure);

pub fn main() {
    let s = Structure(4);
    let d = Deep(&s);
    println!("This is the Structure struct : {:?}", s);
    println!("This is the Deep struct: {:#?}", d);
}

