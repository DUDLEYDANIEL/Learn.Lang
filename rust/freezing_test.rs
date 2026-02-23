fn main() {
    let mut _mut_int = 7i32;

    {
        let _mut_int = _mut_int;
        // the _mut_int is froze due to the immut
        // _mut_int = 30;
    }

    _mut_int = 3;
}
