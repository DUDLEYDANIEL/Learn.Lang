struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String)
    -> User{
        User{
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
}


pub fn fn main() {
    let user1 = User{
        active: false,
        username: String::from("kishoreP"),
        email:: String::from("kishorep@gmail.com")
        sign_in_count: 1,
    };

    let user2 = User{
        email: String::from("kishore2005@gmail.com"),
        ..user1
    };

}

