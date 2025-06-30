use std::fmt::Debug;
use container_traits::{ IntoInner, TryAccept};
use matrix_traits::{Matrix, MatrixNotTall, MatrixWide, MatrixTryConstruct, Transpose, MatrixConstructError, TryFromMatrix};
use super::{tall::Tall, NotTall, Square};

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
pub struct Wide<M:Matrix>(M);



fn test_into_dyn<F:nalgebra::Scalar>(m:&Wide<nalgebra::DMatrix<F>>) -> nalgebra::DMatrix<F> {
    use matrix_traits::IntoDynMatrix;
    m.clone().into_dyn_matrix()
}

impl<M:Matrix> MatrixNotTall for Wide<M> {}
impl<M:Matrix> MatrixWide    for Wide<M> {}

impl<M:Matrix+Transpose<Output=MT>,MT:MatrixTryConstruct> Transpose for Wide<M> {
    type Output=Tall<MT>;

    fn transpose(self) -> Self::Output {
        Tall::try_from_matrix(self.0.transpose()).ok().unwrap()
    }
}

impl<M:MatrixTryConstruct> TryFrom<NotTall<M>> for Wide<M> {
    type Error=Square<M>;
    fn try_from(m: NotTall<M>) -> Result<Self, Self::Error> {
        let m=m.into_inner();
        if m.is_square() {
            Err(Square::try_from_matrix(m).ok().unwrap())
        } else {
            Ok(Wide::try_from_matrix(m).ok().unwrap())
        }
    }
}

impl<M:MatrixTryConstruct> TryAccept<U2,M::T,MatrixConstructError> for Wide<M> {
    fn try_accept<'a>((nrows,ncols):U2,_:impl Fn(U2) -> &'a M::T) -> Result<(),MatrixConstructError> where M::T: 'a {
        if nrows < ncols {
            Ok(())
        } else {
            Err(MatrixConstructError::DimensionMismatch)
        }
    }
}