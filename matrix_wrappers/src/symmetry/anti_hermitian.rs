
use algebra_traits::ComplexNumber;
use container_traits::{TryAccept,  IntoIter, IntoParameters};
use matrix_traits::{Matrix, MatrixConstructError, MatrixSquare, MatrixSquareTryConstruct};
use utils::iter::IntoExactSizeIterator;


// for exp
// use algebra_traits::Exp;
// use crate::Unitary;
// use matrix_traits::{AlgebraMatrix, IntoMatrix};
// use algebra::Complex;

type U2=(usize,usize);

#[derive(Clone, Debug, PartialEq,
         algebra_derive::ScalarContainer,
         container_derive::JustContainer,
         container_derive::IntoInner,
         matrix_derive::Inherit,
         matrix_derive::ClosedTranspose,
         matrix_derive::MatrixShape,
         derive_more::Index)]
pub struct AntiHermitian<M:MatrixSquare>(M) where M::T:Clone+ComplexNumber;

impl<F : Clone+ComplexNumber,
     M : MatrixSquareTryConstruct<T=F>> TryAccept<U2,F,MatrixConstructError> for AntiHermitian<M> {
    fn try_accept<'a>((nrows,ncols):U2,f:impl Fn(U2) -> &'a F) -> Result<(),MatrixConstructError> where F: 'a {
        if nrows != ncols { return Err(MatrixConstructError::DimensionMismatch); }
        for i in 0..nrows {
            for j in 0..i {
               if f((i,j)) != &-f((j,i)).clone().conjugate() {
                  return Err(MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType);
               }
            }
        }
        Ok(())
    }
}

// impl<M:MatrixSquareTryConstruct+AlgebraMatrix> Exp for AntiHermitian<M> where M::T:ComplexNumber+'static {
//     type Output=Unitary<M>;
//     fn exp(self) -> Self::Output {
//         let m:MatrixDyn<M::T>=self.into_matrix();
//         let m:MatrixDyn::<Complex<M::T>>=m.map(|r|r.into());
//         // m.diagonalize()
//         //  .map_eig_vals(Complex::exp)
//         //  .into_matrix()
//            m.map(|c|c.into_real())
//             .try_into_matrix().ok().unwrap()
//     }
// }



impl<M : MatrixSquare<T=F>,
     F : ComplexNumber> IntoParameters<F> for AntiHermitian<M> {
    fn into_parameters(self) -> impl ExactSizeIterator<Item=F> {
        let n=self.n();
        let len=n*(n-1)/2;
        self.into_rows()
            .enumerate()
            .map(|(i,r)|r.into_iterator().skip(i+1))
            .flatten()
            .into_exact_size_iter(len)
    }
}