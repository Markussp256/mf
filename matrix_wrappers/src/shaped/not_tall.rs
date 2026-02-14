use std::fmt::Debug;
use container_traits::{OtherDimensionMismatchError, TryAccept};

use matrix_traits::{Matrix, MatrixConstructError, MatrixViewNotTall, MatrixTryConstruct, MatrixView, Transpose, TryFromMatrix};

use super::not_wide::NotWide;

type U2=(usize,usize);

#[derive(Clone,
         Debug,
         PartialEq,
         algebra_derive::ScalarContainer,
         container_derive::ChangeT,
         container_derive::ContainerViewMut,
         container_derive::TryIntoElement,
         container_derive::IntoIterator,
         container_derive::IntoIterIndexed,
         container_derive::NewUnchecked,
         container_derive::IntoInner,
         derive_more::AsRef,
         derive_more::Index,
         derive_more::IndexMut,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::ClosedMatrixMatrixProduct,
         matrix_derive::MatrixViewNotWide,
         matrix_derive::MatrixViewSquare,
         matrix_derive::MatrixViewWide,
         matrix_derive::PopRow,
)]
pub struct NotTall<M:MatrixView>(M);

impl<M:MatrixView> MatrixViewNotTall for NotTall<M> {}

impl<M:MatrixView+Transpose<Output=MT>,MT:MatrixTryConstruct> Transpose for NotTall<M> {
    type Output=NotWide<MT>;

    fn into_transpose(self) -> Self::Output {
        NotWide::try_from_matrix(self.0.into_transpose()).ok().unwrap()
    }
}

impl<M:MatrixTryConstruct> TryAccept<U2,M::T,MatrixConstructError> for NotTall<M> {
    fn try_accept<'a>((nrows,ncols):U2,_:impl Fn(U2) -> &'a M::T) -> Result<(),MatrixConstructError> where M::T: 'a {
        if nrows <= ncols {
            Ok(())
        } else {
            Err(OtherDimensionMismatchError.into())
        }
    }
}