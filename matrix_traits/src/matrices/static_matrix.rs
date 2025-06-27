use crate::{Matrix,MatrixSquare};

pub trait StaticMatrix : Matrix {
    const M:usize;
    const N:usize;
}

pub trait SquareStaticMatrix : MatrixSquare {
    const M:usize;
}