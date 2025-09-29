use crate::Matrix;

pub trait CropToSquareMatrixIfWide : Matrix {
    type Output : Matrix<T=Self::T,Col=Self::Col>;
    fn crop_to_square_matrix_if_wide(self) -> Self::Output;
}

pub trait CropToSquareMatrixIfTall : Matrix {
    type Output : Matrix<T=Self::T,Row=Self::Row>;
    fn crop_to_square_matrix_if_tall(self) -> Self::Output;
}

pub trait CropToSquareMatrix : Matrix {
    type Output : Matrix<T=Self::T>; // its actually square but we can not implement that because Square struct is in matrix_wrapper but we need to implement in this crate for nalgebra
    fn crop_to_square_matrix(self) -> Self::Output;
}



pub trait SquareRowSize : Matrix {
    type Output : Matrix<T=Self::T>;
}

pub trait SquareColSize : Matrix {
    type Output : Matrix<T=Self::T>;
}