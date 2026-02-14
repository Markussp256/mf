use std::fmt::Debug;
use container_traits::{IntoInner, OtherDimensionMismatchError, TryAccept};
use matrix_traits::{Matrix, MatrixView, MatrixViewNotWide, MatrixViewTall, MatrixConstructError, MatrixTryConstruct, Transpose, TryFromMatrix};

use super::{wide::Wide, NotWide, Square};

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
         matrix_derive::Inherit,
         matrix_derive::PopCol,
)]
pub struct Tall<M:MatrixView>(M);

impl<M:MatrixView> MatrixViewNotWide for Tall<M> {}
impl<M:MatrixView> MatrixViewTall    for Tall<M> {}

impl<M:MatrixView+Transpose<Output=MT>,MT:MatrixTryConstruct> Transpose for Tall<M> {
    type Output=Wide<MT>;

    fn into_transpose(self) -> Self::Output {
        Wide::try_from_matrix(self.0.into_transpose()).ok().unwrap()
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
            Err(OtherDimensionMismatchError.into())
        }
    }
}