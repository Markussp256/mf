use crate::Matrix;

pub trait ChangeDim : Matrix {
    type Output<const M:usize,const N:usize> : Matrix<T=Self::T>;
}