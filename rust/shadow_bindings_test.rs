fn main() {
    let shadow_bind = 1;

    {
        println!("before beiing shadowed: {}", shadow_bind);
        let shadow_bind = "abc";

        println!("shadow in inner block: {}", shadow_bind);
    }
    let shadow_bind = 2;

    println!("shadow in outer block: {}", shadow_bind);
}
