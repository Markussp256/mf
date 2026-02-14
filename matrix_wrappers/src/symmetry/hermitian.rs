use algebra_traits::ComplexNumber;
use container_traits::{LensNotEqualError, TryAccept};
use matrix_traits::{MatrixConstructError, MatrixViewSquare};

type U2=(usize,usize);

#[derive(Clone, Debug, PartialEq,
         algebra_derive::ScalarContainer,
         container_derive::JustContainer,
         container_derive::NewUnchecked,
         container_derive::IntoInner,
         derive_more::AsRef,
         derive_more::Index,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::MatrixNormal,
         matrix_derive::ClosedTranspose,
         matrix_derive::MatrixShape,
)]
pub struct Hermitian<M:MatrixViewSquare>(M) where M::T : Clone+ComplexNumber;

// crate::macros::as_square_into_square!(Hermitian);


impl<F:Clone+ComplexNumber, M:MatrixViewSquare<T=F>> TryAccept<U2,F,MatrixConstructError> for Hermitian<M> {
    fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
        LensNotEqualError::try_new(nrows, ncols)?;
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