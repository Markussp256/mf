use std::fmt::Debug;
use container_traits::TryAccept;
use matrix_traits::{Matrix, MatrixNotWide, MatrixConstructError, MatrixTryConstruct, TryFromMatrix, Transpose};
use super::not_tall::NotTall;

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
         matrix_derive::MatrixNotTall,
         matrix_derive::MatrixSquare,
         matrix_derive::MatrixTall,
)]
pub struct NotWide<M:Matrix>(M);

impl<M:Matrix> MatrixNotWide for NotWide<M> {}

impl<M:Matrix+Transpose<Output=MT>,MT:MatrixTryConstruct> Transpose for NotWide<M> {
    type Output=NotTall<MT>;

    fn transpose(self) -> Self::Output {
        NotTall::try_from_matrix(self.0.transpose()).ok().unwrap()
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