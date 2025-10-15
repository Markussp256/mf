use std::fmt::Debug;
use container_traits::TryAccept;
use matrix_traits::{IntoTranspose, Matrix, MatrixConstructError, MatrixNotWide, MatrixTryConstruct, MatrixView, Transpose, TryFromMatrix};
use super::not_tall::NotTall;

type U2=(usize,usize);

#[derive(Clone,
         Debug,
         PartialEq,
         algebra_derive::ScalarContainer,
         container_derive::ChangeT,
         container_derive::TryIntoElement,
         container_derive::IntoIterator,
         container_derive::IntoIterIndexed,
         container_derive::ContainerViewMut,
         container_derive::NewUnchecked,
         container_derive::IntoInner,
         derive_more::AsRef,
         derive_more::Index,
         derive_more::IndexMut,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::ClosedMatrixMatrixProduct,
         matrix_derive::MatrixNotTall,
         matrix_derive::MatrixSquare,
         matrix_derive::MatrixTall,
         matrix_derive::PopCol,
)]
pub struct NotWide<M:MatrixView>(M);

impl<M:MatrixView> MatrixNotWide for NotWide<M> {}

impl<M:MatrixView+Transpose<Output=MT>,MT:MatrixTryConstruct> Transpose for NotWide<M> {
    type Output=NotTall<MT>;

    fn transpose(&self) -> Self::Output {
        NotTall::try_from_matrix(self.0.transpose()).ok().unwrap()
    }
}

impl<M:MatrixView+IntoTranspose<Output=MT>,MT:MatrixTryConstruct> IntoTranspose for NotWide<M> {
    type Output=NotTall<MT>;

    fn into_transpose(self) -> Self::Output {
        NotTall::try_from_matrix(self.0.into_transpose()).ok().unwrap()
    }
}

impl<M:MatrixTryConstruct> TryAccept<U2,M::T,MatrixConstructError> for NotWide<M> {
    fn try_accept<'a>((nrows,ncols):U2,_:impl Fn(U2) -> &'a M::T) -> Result<(),MatrixConstructError> where M::T: 'a {
        if nrows >= ncols {
            Ok(())
        } else {
            Err(MatrixConstructError::DimensionMismatch)
        }
    }
}