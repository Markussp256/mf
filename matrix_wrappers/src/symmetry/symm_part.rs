use algebra_traits::{ComplexNumber, Conjugate, RealNumber, div_by_small_natural::Div2};

use container_traits::TryFromFn;
use matrix_traits::{IntoBaseSquareMatrix, MatrixConstructError, MatrixSquare, MatrixSquareTryConstruct};
use super::{Hermitian,Symmetric};

type U2=(usize,usize);

macro_rules! impl_skew_or_anti_herm {
    ($name:ident, $name_m:ident, $fn_name:ident, $t:ident $(, $conj:ident)? ) => {
        pub trait $name : MatrixSquare+IntoBaseSquareMatrix where Self::T : $t,
            <Self as IntoBaseSquareMatrix>::Output : MatrixSquareTryConstruct<T=Self::T> {
            fn $fn_name(self) -> $name_m<<Self as IntoBaseSquareMatrix>::Output> {
                <$name_m<_> as TryFromFn<U2,Self::T,MatrixConstructError>>::try_from_fn(
                    self.matrix_dimensions(),
                    |(i,j)|
                    (self.get((i,j)).unwrap().clone()
                    +self.get((j,i)).unwrap().clone()$(.$conj())?).div2()
                ).unwrap()
            }
        }
        impl<T   : $t,
             M   : MatrixSquare<T=T>+IntoBaseSquareMatrix<Output=Out>,
             Out : MatrixSquareTryConstruct<T=T>> $name for M {}
    };
}
impl_skew_or_anti_herm!(SymmetricPart, Symmetric, symm_part, RealNumber);
impl_skew_or_anti_herm!(HermitianPart, Hermitian, herm_part, ComplexNumber, conjugate);