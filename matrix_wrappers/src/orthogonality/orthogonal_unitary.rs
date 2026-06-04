use algebra_traits::{RealNumber, ComplexNumber, Conjugate};
use container_traits::{IntoInner, LensNotEqualError, NewUnchecked, TryAccept, ItemT};
use matrix_traits::{AlgebraMatrix, MatrixConstructError, MatrixSquareTryConstruct, MatrixViewSquare, Transpose, TryPopCol};

use super::Stiefel;

type U2=(usize,usize);

macro_rules! def_orthogonal_or_unitary {
    ($uc:ident, $tr:ident) => {
        paste::paste!(
       #[derive(Clone, Debug, PartialEq,
         algebra_derive::ClosedConjugate,
         algebra_derive::ClosedNeg,
         container_derive::IntoInner,
         container_derive::Inner,
         container_derive::JustContainer,
         container_derive::NewUnchecked,
         derive_more::AsRef,
         derive_more::Index,
         matrix_derive::Identity,
         matrix_derive::Inherit,
         matrix_derive::MatrixNormal,
         matrix_derive::ClosedMatrixMatrixProduct,
         matrix_derive::MatrixMatrixProductRightTriangular,
         matrix_derive::ClosedTranspose,
         matrix_derive::MatrixShape)]
        pub struct $uc<M:MatrixViewSquare>(M) where M::T : $tr;

        impl<M:MatrixViewSquare> $uc<M> where M::T : $tr {
            pub fn from_stiefel(m:Stiefel<M>) -> Self {
                Self(m.into_inner())
            }
        }

        impl<M:MatrixViewSquare+Conjugate<Output=M>+Transpose<Output=M>> algebra_traits::Inv for $uc<M> where M::T : $tr {
            type Output=Self;
            fn inv(self) -> Self {
                self.into_conjugate_transpose()
            }
        }

        impl<M:MatrixViewSquare+Conjugate<Output=M>+Transpose<Output=M>> algebra_traits::TryInv for $uc<M> where M::T : $tr {
            type Output=Self;
            type Error=();
            fn is_invertible(&self) -> Result<(),()> { Ok(()) }
            fn try_inv(self) -> Result<Self,()> {
                Ok(self.into_conjugate_transpose())
            }
        }

        impl<F:$tr, M: AlgebraMatrix+MatrixSquareTryConstruct<T=F>> TryAccept<U2,F,MatrixConstructError> for $uc<M> {
            fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a M::T) -> Result<(),MatrixConstructError> where M::T: 'a {
                LensNotEqualError::try_new(nrows,ncols)?;
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

        impl<F : $tr, M:MatrixSquareTryConstruct<T=F>+TryPopCol> TryPopCol for $uc<M> {
            type Output=Stiefel<<M as TryPopCol>::Output>;
            fn try_pop_col(self) -> Option<(<Self as TryPopCol>::Output,M::Col)> {
                self.0
                    .try_pop_col()
                    .map(|(ms,col)|(Stiefel::new_unchecked(ms),col))
            }
        }

        );
    };
}


// def_orthogonal_or_unitary!(Orthogonal,    RealNumber);
def_orthogonal_or_unitary!(Unitary,       ComplexNumber);

       #[derive(
        Clone,
        Debug,
        PartialEq,
        algebra_derive::Conjugate,
        algebra_derive::ClosedNeg,
        container_derive::IntoInner,
        container_derive::Inner,
        container_derive::JustContainer,
        container_derive::NewUnchecked,
        derive_more::AsRef,
        derive_more::Index,
        matrix_derive::Identity,
        matrix_derive::Inherit,
        matrix_derive::MatrixNormal,
        matrix_derive::ClosedMatrixMatrixProduct,
        matrix_derive::MatrixMatrixProductRightTriangular,
        matrix_derive::ClosedTranspose,
        matrix_derive::MatrixShape)]
        pub struct Orthogonal<M:MatrixViewSquare>(M) where <M as ItemT>::T : RealNumber;

        impl<M:MatrixViewSquare> Orthogonal<M> where M::T : RealNumber {
            pub fn from_stiefel(m:Stiefel<M>) -> Self {
                Self(m.into_inner())
            }
        }

        impl<M:MatrixViewSquare
              +Conjugate<Output=M>
              +Transpose<Output=M>> algebra_traits::Inv for Orthogonal<M> where M::T : RealNumber {
            type Output=Self;
            fn inv(self) -> Self {
                self.into_conjugate_transpose()
            }
        }

        impl<M:MatrixViewSquare+Conjugate<Output=M>+Transpose<Output=M>> algebra_traits::TryInv for Orthogonal<M> where M::T : RealNumber {
            type Output=Self;
            type Error=();
            fn is_invertible(&self) -> Result<(),()> { Ok(()) }
            fn try_inv(self) -> Result<Self,()> {
                Ok(self.into_conjugate_transpose())
            }
        }

        impl<F:RealNumber, M: AlgebraMatrix+MatrixSquareTryConstruct<T=F>> TryAccept<U2,F,MatrixConstructError> for Orthogonal<M> {
            fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a M::T) -> Result<(),MatrixConstructError> where M::T: 'a {
                LensNotEqualError::try_new(nrows,ncols)?;
                Stiefel::<M>::try_accept((nrows,ncols),f)
            }
        }

        impl<F : RealNumber, M:MatrixSquareTryConstruct<T=F>> Orthogonal<M> {
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

        impl<F : RealNumber, M:MatrixSquareTryConstruct<T=F>+TryPopCol> TryPopCol for Orthogonal<M> {
            type Output=Stiefel<<M as TryPopCol>::Output>;
            fn try_pop_col(self) -> Option<(<Self as TryPopCol>::Output,M::Col)> {
                self.0
                    .try_pop_col()
                    .map(|(ms,col)|(Stiefel::new_unchecked(ms),col))
            }
        }