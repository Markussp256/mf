use crate::Matrix;

pub trait PushRow : Matrix {
    type Output : Matrix<T=Self::T>;
    fn push_row(self,row:Self::Row) -> Self::Output;
}


pub trait PushCol : Matrix {
    type Output : Matrix<T=Self::T>;
    fn push_col(self,col:Self::Col) -> Self::Output;
}

pub trait TryPushRow : Matrix {
    type Output: Matrix<T=Self::T>;
    fn try_push_row(self,row:Self::Row) -> Result<Self::Output, Self::Row>;
}


pub trait TryPushCol : Matrix {
    type Output: Matrix<T=Self::T>;
    fn try_push_col(self,col:Self::Col) -> Result<Self::Output, Self::Col>;
}