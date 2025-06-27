use std::fmt::Debug;
use container_traits::TryAccept;

use matrix_traits::{Matrix, MatrixConstructError, MatrixNotTall, MatrixTryConstruct, Transpose, TryFromMatrix};

use super::not_wide::NotWide;

type U2=(usize,usize);

#[derive(Clone,
         Debug,
         PartialEq,
         algebra_derive::ScalarContainer,
         container_derive::ContainerMut,
         container_derive::IntoInner,
         derive_more::AsRef,
         derive_more::Index,
         derive_more::IndexMut,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::MatrixNotWide,
         matrix_derive::MatrixSquare,
         matrix_derive::MatrixWide,
)]
pub struct NotTall<M:Matrix>(M);

impl<M:Matrix> MatrixNotTall for NotTall<M> {}

impl<M:Matrix+Transpose<Output=MT>,MT:MatrixTryConstruct> Transpose for NotTall<M> {
    type Output=NotWide<MT>;

    fn transpose(self) -> Self::Output {
        NotWide::try_from_matrix(self.0.transpose()).ok().unwrap()
    }
}

impl<M:MatrixTryConstruct> TryAccept<U2,M::T,MatrixConstructError> for NotTall<M> {
    fn try_accept<'a>((nrows,ncols):U2,_:impl Fn(U2) -> &'a M::T) -> Result<(),MatrixConstructError> where M::T: 'a {
        if nrows <= ncols {
            Ok(())
        } else {
            Err(MatrixConstructError::DimensionMismatch)
        }
    }
}