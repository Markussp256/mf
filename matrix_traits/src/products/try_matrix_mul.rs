use crate::*;
use container_traits::{IntoSum, IntoVec};
use std::ops::Mul;

use num_traits::Zero;

pub trait TryMatrixMul<Rhs:Matrix> : Matrix {
    fn try_matrix_mul<Out:MatrixTryConstruct<T=<Self::T as Mul<Rhs::T>>::Output>> (self, rhs:Rhs) -> Option<Out>
    where Self::T : Mul<Rhs::T>;
}

impl<M      : Matrix<T=F>,
     Rhs    : Matrix<T=F2>,
     F      : Clone+Mul<F2,Output=F3>,
     F2     : Clone,
     F3     : Zero> TryMatrixMul<Rhs> for M {
        fn try_matrix_mul<Out:MatrixTryConstruct<T=F3>> (self, rhs:Rhs) -> Option<Out> {
            if self.ncols() != rhs.nrows() { return None; }
            Out::try_from_fn((self.nrows(),rhs.ncols()),|(i,j)|
               self.row(i).unwrap().into_vec().into_iter()
                   .zip(rhs.col(j).unwrap().into_vec().into_iter())
                   .map(|(l,r)|l*r)
                   .into_sum()
            ).ok()
        }
}