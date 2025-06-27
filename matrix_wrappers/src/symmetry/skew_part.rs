use algebra_traits::{ComplexNumber, Conjugate, RealNumber, div_by_small_natural::Div2};

use container_traits::TryFromFn;
use matrix_traits::{IntoBaseSquareMatrix, MatrixConstructError, MatrixSquare, MatrixSquareTryConstruct};
use super::{AntiHermitian,SkewSymmetric};

type U2=(usize,usize);

macro_rules! impl_skew_or_anti_herm {
    ($name:ident, $m_name:ident, $fn_name:ident, $t:ident $(, $conj:ident)? ) => {
        pub trait $name : MatrixSquare+IntoBaseSquareMatrix where Self::T : $t,
            <Self as IntoBaseSquareMatrix>::Output : MatrixSquareTryConstruct<T=Self::T> {
            fn $fn_name(self) -> $m_name<<Self as IntoBaseSquareMatrix>::Output> {
                <$m_name<_> as TryFromFn<U2,Self::T,MatrixConstructError>>::try_from_fn(
                    self.matrix_dimensions(),
                    |(i,j)|
                    (self.get((i,j)).unwrap().clone()
                    -self.get((j,i)).unwrap().clone()$(.$conj())?).div2()
                ).unwrap()
            }
        }
        impl<T   : $t,
             M   : MatrixSquare<T=T>+IntoBaseSquareMatrix<Output=Out>,
             Out : MatrixSquareTryConstruct<T=T>> $name for M {}
    };
}
// impl_skew_or_anti_herm!(GenAntiHermitianPart, AntiHermitian, skew_part, Scalar);
impl_skew_or_anti_herm!(SkewSymmetricPart, SkewSymmetric, skew_part, RealNumber);
impl_skew_or_anti_herm!(AntiHermitianPart, AntiHermitian, skew_part, ComplexNumber, conjugate);
