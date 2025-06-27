use std::ops::Neg;

use container_traits::TryAccept;
use matrix_traits::{Matrix, MatrixConstructError, MatrixTryConstruct, Transpose, TryFromMatrix};
use num_traits::Zero;
use crate::shaped::square::Square;

type U2=(usize,usize);

#[derive(Clone, Debug, PartialEq,
         algebra_derive::ScalarContainer,
         algebra_derive::Inv,
         container_derive::JustContainer,
         derive_more::AsRef,
         derive_more::Index,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::MatrixMatrixProduct,
         matrix_derive::MatrixShape)]
pub struct LeftTriangular<M:Matrix>(M) where M::T : Zero;

pub type SquareLeftTriangular<M>=LeftTriangular<Square<M>>;

impl<F : Zero,
     M : MatrixTryConstruct<T=F>+Transpose<Output=Mt>,
     Mt: MatrixTryConstruct<T=F>> Transpose for LeftTriangular<M> {
    type Output=super::RightTriangular<Mt>;
    fn transpose(self) -> Self::Output {
        Self::Output::try_from_matrix(self.0.transpose()).ok().unwrap()
    }
}

impl<F : Zero,
     M : MatrixTryConstruct<T=F>> TryAccept<U2,F,MatrixConstructError> for LeftTriangular<M> {
    fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
        for i in 0..nrows {
            for j in (i+1)..ncols {
                if !f((i,j)).is_zero() {
                    return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
                }
            }
        }
        Ok(())
    }
}

impl<F:Neg<Output=F>+Zero,M:MatrixTryConstruct<T=F>> LeftTriangular<M> {
    pub fn try_neg_row(self,i:usize) -> Result<Self,MatrixConstructError> {
        self.0
            .try_neg_row(i) 
            .map(|m|Self(m))
    }

    pub fn try_neg_col(self,j:usize) -> Result<Self,MatrixConstructError> {
        self.0
            .try_neg_col(j)
            .map(|m|Self(m))
    }
}