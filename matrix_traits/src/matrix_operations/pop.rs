use crate::Matrix;

pub trait PopRow : Matrix {
    type Output: Matrix<T=Self::T>;
    fn pop_row(self) -> (Self::Output, Self::Row);
}

pub trait PopCol : Matrix {
    type Output: Matrix<T=Self::T>;
    fn pop_col(self) -> (Self::Output, Self::Col);
}


pub trait TryPopRow : Matrix {
    type Output: Matrix<T=Self::T>;
    fn try_pop_row(self) -> Option<(Self::Output, Self::Row)>;
}


pub trait TryPopCol : Matrix {
    type Output: Matrix<T=Self::T>;
    fn try_pop_col(self) -> Option<(Self::Output, Self::Col)>;
}