use std::ops::Neg;
use container_traits::{Iter, TryAccept};
use matrix_traits::{Matrix, MatrixView, MatrixConstructError, MatrixTryConstruct, Transpose, TryFromMatrix, TryPushCol, TryPushRow};
use num_traits::Zero;

use crate::shaped::square::Square;

type U2=(usize,usize);

#[derive(Clone, Debug, PartialEq,
         algebra_derive::ScalarContainer,
         algebra_derive::ClosedInv,
         container_derive::JustContainer,
         container_derive::NewUnchecked,
         container_derive::IntoInner,
         container_derive::Inner,
         derive_more::AsRef,
         derive_more::Index,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::ClosedMatrixMatrixProduct,
         matrix_derive::MatrixShape,
         matrix_derive::PopRow,
         matrix_derive::PopCol
)]
pub struct RightTriangular<M:MatrixView>(M) where M::T : Zero;

// crate::macros::as_matrix_into_matrix!(RightTriangular);

pub type SquareRightTriangular<M>=RightTriangular<Square<M>>;


impl<F : Zero,
     M : MatrixTryConstruct<T=F>+Transpose<Output=Mt>,
     Mt: MatrixTryConstruct<T=F>> Transpose for RightTriangular<M> {
    type Output=super::LeftTriangular<Mt>;

    fn into_transpose(self) -> Self::Output {
        Self::Output::try_from_matrix(
            self.0
                    .into_transpose()).ok().unwrap()
    }
}

impl<F : Zero,
     M : MatrixTryConstruct<T=F>> TryAccept<U2,F,MatrixConstructError> for RightTriangular<M> {
    fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
        for i in 1..nrows {
            for j in 0..usize::min(i-1,ncols) {
                if !f((i,j)).is_zero() {
                    return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
                }
            }
        }
        Ok(())
    }
}

impl<F:Neg<Output=F>+Zero, M:MatrixTryConstruct<T=F>> RightTriangular<M> {
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

impl<F:Zero, M : Matrix<T=F>+TryPushRow> TryPushRow for RightTriangular<M> {
    type Output=RightTriangular<<M as TryPushRow>::Output>;
    fn try_push_row(self,row:M::Row) -> Result<RightTriangular<<M as TryPushRow>::Output>, M::Row> {
        if !row.iter()
               .take(self.0.nrows())
               .all(|v|v.is_zero()) {
            return Err(row);
        }
        self.0
            .try_push_row(row)
            .map(|m|RightTriangular(m))
    }
}

impl<F:Zero, M : Matrix<T=F>+TryPushCol> TryPushCol for RightTriangular<M> {
    type Output=RightTriangular<<M as TryPushCol>::Output>;
    fn try_push_col(self,col:M::Col) -> Result<RightTriangular<<M as TryPushCol>::Output>, M::Col> {
        if !col.iter()
               .skip(self.0.ncols()+1)
               .all(|v|v.is_zero()) {
            return Err(col);
        }
        self.0
            .try_push_col(col)
            .map(|m|RightTriangular(m))
    }
}