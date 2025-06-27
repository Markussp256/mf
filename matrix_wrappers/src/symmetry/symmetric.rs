use algebra_traits::RealNumber;
use container_traits::TryAccept;
use matrix_traits::{MatrixConstructError, MatrixSquare, MatrixSquareTryConstruct};

type U2=(usize,usize);


#[derive(Clone, Debug, PartialEq,
         algebra_derive::ScalarContainer,
         container_derive::JustContainer,
         container_derive::Map,
         container_derive::IntoInner,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::MatrixShape,
         matrix_derive::ClosedTranspose,
         derive_more::Index)]
pub struct Symmetric<M:MatrixSquare>(M) where M::T : RealNumber;

impl<F : Clone+RealNumber,
     M : MatrixSquareTryConstruct<T=F>> TryAccept<U2,F,MatrixConstructError> for Symmetric<M> {
    fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
        if nrows != ncols { return Err(MatrixConstructError::DimensionMismatch); }
        for i in 0..nrows {
            for j in 0..i {
               if f((i,j)) != &-f((j,i)).clone() {
                  return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
               }
            }
        }
        Ok(())
    }
}