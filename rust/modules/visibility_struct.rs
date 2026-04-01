pub mod library{
    pub struct Book{
        pub title: String, 
        isbn: String,
    }

    impl Book{
        pub fn new(title: &str, isbn: &str) -> Self{
            Self {
                title: title.to_string(),
                isbn: isbn.to_string(),
            }
        }
    }
}

fn main(){
    let my_book = library::Book::new("Rust Guide", "123-456-789");
    println!("{}", my_book.title);
    //cannot print my_book.isbn
}
