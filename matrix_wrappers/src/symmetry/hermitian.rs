use algebra_traits::ComplexNumber;
use container_traits::TryAccept;
use matrix_traits::{MatrixConstructError, MatrixSquare};

type U2=(usize,usize);

#[derive(Clone, Debug, PartialEq,
         algebra_derive::ScalarContainer,
         container_derive::JustContainer,
         container_derive::IntoInner,
         derive_more::AsRef,
         derive_more::Index,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::ClosedTranspose,
         matrix_derive::MatrixShape
)]
pub struct Hermitian<M:MatrixSquare>(M) where M::T : Clone+ComplexNumber;

// crate::macros::as_square_into_square!(Hermitian);


impl<F:Clone+ComplexNumber, M:MatrixSquare<T=F>> TryAccept<U2,F,MatrixConstructError> for Hermitian<M> {
    fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
        if nrows != ncols { return Err(MatrixConstructError::DimensionMismatch); }
        for i in 0..nrows {
            for j in 0..i {
               if f((i,j)) != &f((j,i)).clone().conjugate() {
                  return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
               }
            }
        }
        Ok(())
    }
}