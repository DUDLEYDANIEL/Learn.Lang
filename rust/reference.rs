fn main(){
    let s1: String = String::from("hello");
    let s2: usize= calc_len(&s1);
    println!("The len of {} is {}.", s1, s2);
}

// reference is immutable by default 
// we can modify by making the value mutable and passing the value as a reference
// a value can have one mutable reference in a scope


fn calc_len(s: &String)
    -> usize{
    let length = s.len();
    length
}
