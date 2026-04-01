pub fn root_func(){
    // This is the Ancestor 
    println!("Action from the Ancestor");
}

pub mod parent_lvl{
    pub fn parent_func(){
        println!("Action from parent!");
    }

    pub mod child_lvl{
        pub fn call_family(){
            //calling parent_lvl function 
            super::parent_func();
            //calling ancestor 
            super::super::root_func();
            crate::root_func();
        }
    }
}

fn main(){
    parent_lvl::child_lvl::call_family();
}
