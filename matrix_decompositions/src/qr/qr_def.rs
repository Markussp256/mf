use std::ops::Mul;
use matrix_traits::{TryMatrixMatrixProduct, TryMatrixVectorProduct, ColVectorAnyConstruct, ConjugateTranspose, Matrix, MatrixSolveError, MatrixTryConstruct, Transpose};
use algebra_traits::{TryDiv,TrySolve, Scalar, ComplexNumber, RealNumber};
use container_traits::ChangeT;

macro_rules! def_qr {
    ($name:ident, $tr:ident) => {
        pub trait $name : Matrix
            where Self::T : TryDiv,
            <Self::T as TryDiv>::Output : $tr+Mul<Self::T,Output=Self::T> {
            type Q : Matrix<T=<Self::T as TryDiv>::Output> + TryMatrixMatrixProduct<Self::R>;
            type R : MatrixTryConstruct<T=Self::T>;

            fn qr(self) -> (Self::Q,Self::R);

            fn try_solve_least_squares
            <Rhs : ColVectorAnyConstruct+ChangeT<Mid::T,Output=Mid>,
             Mid : ColVectorAnyConstruct,
             Out : ColVectorAnyConstruct>(self, rhs:Rhs) -> Option<Out>
            where   Self::Q : ConjugateTranspose,
                    <Self::Q as Transpose>::Output : TryMatrixVectorProduct<Rhs,T=<Self::T as TryDiv>::Output>,
                    <Self::T as TryDiv>::Output : Mul<Rhs::T,Output=Mid::T>,
                    Self::R : TrySolve<Mid, MatrixSolveError, Output=Out>,
                    // Mid::T  : TryDiv<Self::T,Output=Out::T>,
                    // Self::T : Mul<Out::T,Output=Mid::T>
                    {
                let (q,r)=self.qr();
                let qt=q.conjugate_transpose();
                let qtr=qt.try_matrix_vector_product::<Mid>(rhs)?;
                r.try_solve(qtr).ok()
            }
        }
    };
}
def_qr!(QR, Scalar);
def_qr!(OrthogonalQR, RealNumber);
def_qr!(UnitaryQR, ComplexNumber);


// macro_rules! impl_try_solve_from_try_solve_least_squares {
//     ($t:ty, $rhs:ty, $out:ty, $e:ty, $trname:ident) => {
        
//     // impl<...> must be implemented before macro
//         algebra_traits::TrySolve<$rhs,$e> for $t
//             where $rhs : ColVectorAnyConstruct<T=Self::T>,
//                   $out : ColVectorAnyConstruct<T=<Self::T as algebra_traits::TryDiv>::Output> {
//         type Output=$out;
//         fn try_solve(self, rhs:$rhs) -> Result<$out,$e> {
//             algebra_traits::LenNotEqualError::try_new(self.nrows(),rhs.len())?;
//             match <$t as $trname>::try_solve_least_squares(self,rhs) {
//                 Some(r) => Ok(r)
//                 None => MatrixNotRegularError.into()
//             }
//         }
//     }
//     }
// }
// impl<F:Scalar> impl_try_solve_from_try_solve_least_squares!(
//     nalgebra::DMatrix<F>,
//     nalgebra::DVector<F>,
//     nalgebra::DVector<F>,
//     either::Either<LenNotEqualError,MatrixNotRegularError>,
//     QR)