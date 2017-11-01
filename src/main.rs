use std::fmt;
struct Matrix(
    f32, f32, f32, f32
);

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {0} {1} )\n( {2} {3} )", &self.0, &self.1, &self.2, &self.3)
    }
}

fn main() {
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    println!("{:?}", tuple_of_tuples);

    let matrix = Matrix(1.12, 2.23, 3.34, 4.45);
    println!("{}", matrix);
    println!("{}", transpose(matrix))
}