use std::fmt::{self, Formatter, Display};

fn rev_tuple(pair: (i32, bool))
    -> (bool, i32) {
        let (int_param, bool_param) = pair;
        (bool_param, int_param)
}



#[derive(Debug)]
struct Matrix(
    f32,f32,f32,f32
);

fn transpose_matrix(mat: &Matrix)
    -> Matrix {
        Matrix(mat.0, mat.2, mat.1, mat.3)
}


impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter)
        -> fmt::Result {
          write!(f, "( {} {} \n\t{} {} )",self.0, self.1, self.2, self.3)
   }
}



pub fn main() {
    
    let pair = (1, true);
    println!("The tuple of pair is {:?}", pair);
    println!("The reversed pair is {:?}", rev_tuple(pair));
    
    let matrix = Matrix(1.1, 2.2, 3.3, 4.4);
    println!("Debug : {:?}", matrix);
    println!("Display: {}", matrix);

    println!("The transpose of an matrix is :");
    println!("Debug: {:?}", transpose_matrix(&matrix));
    println!("Display: {}", transpose_matrix(&matrix));
}
