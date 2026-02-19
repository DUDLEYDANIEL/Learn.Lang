#![allow(dead_code)]

enum Stage{
    Beginner, 
    Advanced,
}

enum Role{
    Student,
    Teacher,
}

pub fn main() {
    use Stage::{
        Beginner,
        Advanced
    };

    use Role::*;

    let stage = Beginner;
    let role = Student;

    match stage{
        Beginner
            => println!("Beginners are starting their learning"),
        Advanced
            => println!("Advanced learners are mastering their subject"),
    }

    match role {
        Student
            => println!("Students are acquiring knowledge!"),
        Teacher
            => println!("Teachers are spreading knowledge!"),
    }
}
