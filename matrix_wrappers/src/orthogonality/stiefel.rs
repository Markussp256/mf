use std::ops::Neg;

use algebra::Unit;
use algebra_traits::{Conjugate, Scalar};
use container_traits::{TryAccept, Len, Get,  IntoInner};
use matrix_traits::{AlgebraMatrix, ColVectorTryConstruct, Matrix, MatrixView, MatrixConstructError, MatrixNormal, MatrixViewNotTall, MatrixViewNotWide, MatrixViewSquare, MatrixViewTall, MatrixTryConstruct};

use utils::kron_delta;
type U2=(usize,usize);

#[derive(Clone, Debug, PartialEq,
        algebra_derive::Conjugate,
        algebra_derive::Neg,
        container_derive::IntoInner,
        container_derive::Inner,
        container_derive::JustContainer,
        container_derive::NewUnchecked,
        derive_more::AsRef,
        derive_more::Index,
        matrix_derive::Identity,
        matrix_derive::ClosedMatrixMatrixProduct,
        matrix_derive::Transpose,
        matrix_derive::MatrixMatrixProductRightTriangular,
        matrix_derive::Inherit,
        matrix_derive::PopCol)]
pub struct Stiefel<M:MatrixView>(M);

pub type SquareStiefel<M> = Stiefel<crate::Square<M>>;

// Stiefel matrix can not be wide, otherwise inherit
impl<M:MatrixView>    MatrixViewNotWide for Stiefel<M> {}
impl<M:MatrixViewNotTall> MatrixViewNotTall for Stiefel<M> {}
impl<M:MatrixViewSquare>  MatrixViewSquare  for Stiefel<M> {}
impl<M:MatrixViewTall>    MatrixViewTall    for Stiefel<M> {}

impl<M:MatrixViewSquare+Conjugate<Output=M>>  MatrixNormal  for Stiefel<M> {}

// impl<M:Matrix+Transpose<Output=Mt>,Mt> Transpose for Stiefel<M> {
//     type Output=Mt;
//     fn transpose(&self) -> Self::Output {
//         self.0
//             .transpose()
//     }
// }

// impl<M:Matrix+IntoTranspose<Output=Mt>,Mt> IntoTranspose for Stiefel<M> {
//     type Output=Mt;
//     fn into_transpose(self) -> Self::Output {
//         self.0
//             .into_transpose()
//     }
// }

impl<F:Scalar, M: AlgebraMatrix+MatrixTryConstruct<T=F>> TryAccept<U2,F,MatrixConstructError> for Stiefel<M> {
    fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
        if ncols > nrows { return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType); }
        let m=M::try_from_fn((nrows,ncols),|ind|f(ind).clone())?;
        let is_acc=
            (0..ncols).map(|j|std::iter::repeat(j).enumerate().take(ncols))
                      .flatten()
                      .map(|(i,j)|(m.try_col_sc_prod(i,j).unwrap(),kron_delta(i,j)))
                      .all(|(l,r):(F,F)|l.is_close_to(r));
        if is_acc {
            Ok(())
        } else {
            Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
        }
    }
}

impl<F:Neg<Output=F>, M:MatrixTryConstruct<T=F>> Stiefel<M> {
    pub fn try_neg_col(self,j:usize) -> Result<Self,MatrixConstructError> {
        self.0
            .try_neg_col(j)
            .map(|m|Self(m))
    }
}

impl<F:Scalar, Col : ColVectorTryConstruct<T=F>, M : AlgebraMatrix+MatrixTryConstruct<T=F,Col=Col>> TryFrom<Unit<Col>> for Stiefel<M> {
    type Error=Unit<Col>;
    fn try_from(value: Unit<Col>) -> Result<Self, Self::Error> {
        if M::try_accept((value.len(),1),|(i,_)|value.get(i).unwrap()).is_ok() {
            let col=value.into_inner();
            Ok(Self::try_from_cols(std::iter::once(col)).unwrap())
        } else {
            Err(value)
        }
    }
}
