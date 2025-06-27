use algebra_traits::{RealNumber, ComplexNumber};
use container_traits::{TryAccept,  IntoInner};
use matrix_traits::{AlgebraMatrix, MatrixConstructError, MatrixSquareTryConstruct, MatrixSquare, Transpose};

use super::Stiefel;

type U2=(usize,usize);

macro_rules! def_orthogonal_or_unitary {
    ($uc:ident, $tr:ident) => {
        paste::paste!(
       #[derive(Clone, Debug, PartialEq,
         algebra_derive::Conjugate,
         algebra_derive::ClosedNeg,
         container_derive::IntoInner,
         container_derive::JustContainer,
         derive_more::AsRef,
         derive_more::Index,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::MatrixMatrixProduct,
         matrix_derive::ClosedTranspose,
         matrix_derive::MatrixShape)]
        pub struct $uc<M:MatrixSquare>(M) where M::T : $tr;

        impl<M:MatrixSquare> $uc<M> where M::T : $tr {
            pub fn from_stiefel(m:Stiefel<M>) -> Self {
                Self(m.into_inner())
            }
        }

        impl<M:MatrixSquare+Transpose<Output=M>> algebra_traits::Inv for $uc<M> where M::T : $tr {
            type Output=Self;
            fn inv(self) -> Self {
                self.transpose()
            }
        }

        impl<F:$tr, M: AlgebraMatrix+MatrixSquareTryConstruct<T=F>> TryAccept<U2,F,MatrixConstructError> for $uc<M> {
            fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a M::T) -> Result<(),MatrixConstructError> where M::T: 'a {
                if nrows != ncols { return Err(MatrixConstructError::DimensionMismatch); }
                Stiefel::<M>::try_accept((nrows,ncols),f)
            }
        }

        impl<F : $tr, M:MatrixSquareTryConstruct<T=F>> $uc<M> {
            pub fn try_neg_row(self,i:usize) -> Result<Self,MatrixConstructError> {
                self.0
                    .try_neg_row(i)
                    .map(|m|Self(m))
            }

            pub fn try_neg_col(self,j:usize) -> Result<Self,MatrixConstructError>  {
                self.0
                    .try_neg_col(j)
                    .map(|m|Self(m))
            }
        }
        );
    };
}
def_orthogonal_or_unitary!(Orthogonal,    RealNumber);
def_orthogonal_or_unitary!(Unitary,       ComplexNumber);