use num_traits::Zero;


use algebra_traits::RealNumber;
use container_traits::{ IntoIter, IntoParameters, LenTooSmallError, TryAccept};
use matrix_traits::{Matrix, MatrixConstructError, MatrixMut, MatrixSquare, MatrixSquareTryConstruct, SquareStaticMatrix};
use utils::iter::IntoExactSizeIterator;


// for exp
// use algebra_traits::Exp;
// use algebra::Complex;
// use crate::Orthogonal;
// use matrix_traits::{AlgebraMatrix, IntoMatrix};

type U2=(usize,usize);

#[derive(Clone, Debug, PartialEq,
         algebra_derive::ScalarContainer,
         container_derive::JustContainer,
         container_derive::IntoInner,
         matrix_derive::Inherit,
         matrix_derive::ClosedTranspose,
         matrix_derive::MatrixShape,
         derive_more::Index)]
pub struct SkewSymmetric<M:MatrixSquare>(M) where M::T:RealNumber;

impl<F : Clone+RealNumber,
     M : MatrixSquareTryConstruct<T=F>> TryAccept<U2,F,MatrixConstructError> for SkewSymmetric<M> {
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

// impl<M:MatrixSquareTryConstruct+AlgebraMatrix> Exp for SkewSymmetric<M> where M::T:RealNumber+'static {
//     type Output=Orthogonal<M>;
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
     F : RealNumber> IntoParameters<F> for SkewSymmetric<M> {
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

impl<M : SquareStaticMatrix+MatrixSquareTryConstruct<T=F>+MatrixMut<T=F>+Zero, F:Clone+RealNumber>
container_traits::for_static::TryFromParameters<F,MatrixConstructError>
for SkewSymmetric<M> {
    fn try_take_away<I: Iterator<Item=F>>(iter:& mut I) -> Result<Self,MatrixConstructError> {
        let mut m=M::zero();
        let required_len=M::M*(M::M-1)/2;
        let mut len=0;
        for i in 0..M::M {
            for j in i+1..M::M {
                let val=iter.next().ok_or(LenTooSmallError::new(required_len, len))?;
                len += 1;
                *m.get_mut((i,j)).unwrap()=val.clone();
                *m.get_mut((j,i)).unwrap()=-val;
            }
        }
        Self::try_new(m)
    }

    container_traits::try_from_parameters_impl!(F,MatrixConstructError);
}