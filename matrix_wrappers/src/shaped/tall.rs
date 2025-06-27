use std::fmt::Debug;
use container_traits::{IntoInner, TryAccept};
use matrix_traits::{Matrix, MatrixNotWide, MatrixTall, MatrixConstructError, MatrixTryConstruct, Transpose, TryFromMatrix};

use super::{wide::Wide, NotWide, Square};

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
         matrix_derive::Inherit,
)]
pub struct Tall<M:Matrix>(M);

impl<M:Matrix> MatrixNotWide for Tall<M> {}
impl<M:Matrix> MatrixTall    for Tall<M> {}

impl<M:Matrix+Transpose<Output=MT>,MT:MatrixTryConstruct> Transpose for Tall<M> {
    type Output=Wide<MT>;

    fn transpose(self) -> Self::Output {
        Wide::try_from_matrix(self.0.transpose()).ok().unwrap()
    }
}

impl<M:MatrixTryConstruct> TryFrom<NotWide<M>> for Tall<M> {
    type Error=Square<M>;
    fn try_from(m: NotWide<M>) -> Result<Self, Self::Error> {
        let m=m.into_inner();
        if m.is_square() {
            Err(Square::try_from_matrix(m).ok().unwrap())
        } else {
            Ok(Tall::try_from_matrix(m).ok().unwrap())
        }
    }
}


impl<M:MatrixTryConstruct> TryAccept<U2,M::T,MatrixConstructError> for Tall<M> {
    fn try_accept<'a>((nrows,ncols):U2,_:impl Fn(U2) -> &'a M::T) -> Result<(),MatrixConstructError> where M::T: 'a {
        if nrows > ncols {
            Ok(())
        } else {
            Err(MatrixConstructError::DimensionMismatch)
        }
    }
}