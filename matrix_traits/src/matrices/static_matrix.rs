use crate::{Matrix,MatrixViewSquare};

pub trait StaticMatrix : Matrix {
    const M:usize;
    const N:usize;
}

pub trait SquareStaticMatrixView : MatrixViewSquare {
    const M:usize;
}