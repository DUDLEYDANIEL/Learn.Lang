mod my_mod{
    fn private_func(){
        println!("This is a private function");
    }

    pub fn func(){
        println!("This is the public fn of my_mod()");
    }
    
    pub fn indir_acces(){
        println!("calling the private fn through pub fn");
        private_func();
    }

    //nested modules 
    pub mod nested{
        pub fn func(){
            println!("called `my_mod::nested::func()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        pub(in crate::my_mod) fn public_by_path(){
            println!("called `my_mod::nested::public_by_path`");
        }

        pub(self) fn private_by_path(){
            println!("called `my_mod::nested::private_by_path`");
        }

        pub(super) fn public_by_super(){
            println!("called `my_mod::nested::public_by_path`");
        }
    }

    pub fn temp_func(){
    nested::public_by_path();
}
}

fn main(){
    //trying the public functions
    my_mod::func();
    my_mod::indir_acces();
    my_mod::nested::func();
    // my_mod::nested::public_by_path();
    my_mod::temp_func();
}


