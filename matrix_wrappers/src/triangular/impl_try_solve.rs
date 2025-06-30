use std::ops::{Mul, Sub};
use std::fmt::Debug;
use num_traits::Zero;
use algebra_traits::{ClosedTrySub, TryDiv, TrySolve};
use container_traits::{ChangeT, Get, IntoIter, IntoSum, LensNotEqualError, LinearContainerTryConstruct};
use matrix_traits::{TryMatrixVectorProduct, ColVector, ColVectorAnyConstruct, Matrix, MatrixNotRegularError, MatrixSolveError};

use super::RightTriangular;

// since this can fail in any case we only implement try


trait TrySolveApprox<Rhs> {
    type Output;
    fn try_solve_approx(self, b:Rhs) -> Result<Self::Output, MatrixSolveError>;
}

impl<F   : Clone+Zero+Mul<F2,Output=F3>,
     F2  : Clone+Zero,
     F3  : Clone+TryDiv<F,Output=F2>+Zero+Sub<Output=F3>,
     M   : Matrix<T=F>,
     Rhs : LinearContainerTryConstruct<T=F3>+ChangeT<F2,Output=Out>,
     Out : LinearContainerTryConstruct<T=F2>> TrySolveApprox<Rhs> for RightTriangular<M> {
    type Output = Out;
    
    fn try_solve_approx(self, b:Rhs) -> Result<Self::Output, MatrixSolveError> {
        if !self.is_square() { return Err(MatrixSolveError::MatrixNotSquare); }
        let n=self.nrows();
        LensNotEqualError::try_new(n, b.len())?;
        let mut x:Vec<F2>=std::iter::repeat_with(F2::zero).take(n).collect();

        // because
        let vs:Vec<(M::Row,F3)>=
            self.into_rows()
                .zip(b.into_iterator())
                .collect();
        // we can not rev rows because its not double ended iterator
        for (i,(ri,bi)) in vs.into_iter().enumerate().rev() {
            let aii=ri.get(i).unwrap().clone();
            let current: F3=
                ri.into_iterator()
                  .zip(x.iter().cloned())
                  .skip(i+1)
                  .map(|(aij,xj)|aij * xj) // 
                  .into_sum();
            // let desired=b.get(i).unwrap().clone();
            x[i]=(bi-current).try_div(aii).map_err(|_|MatrixNotRegularError)?;
        }
        Ok(Out::any_from_vec(x).unwrap())
    }
}

impl<M   : Matrix,
     Rhs : Clone+ClosedTrySub<Error=ERhs>+ColVectorAnyConstruct, ERhs:Debug,
     Out : Clone+ClosedTrySub<Error=EOut>+ColVector, EOut:Debug>
        TrySolve<Rhs,MatrixSolveError> for RightTriangular<M>
     where  M::T : Zero+Mul<Out::T,Output=Rhs::T>,
            Self : TrySolveApprox<Rhs,Output=Out>
                  +TryMatrixVectorProduct<Out,Output=Rhs>
                  +Matrix<T=M::T>
                  +Clone {
    type Output=Out;
    fn try_solve(self, rhs:Rhs) -> Result<Out, MatrixSolveError> {
        let mut x=self.clone().try_solve_approx(rhs.clone())?;
        for _ in 0..5 {
            let res:Rhs=self.clone().try_matrix_vector_product(x.clone()).unwrap()
                            .try_sub(rhs.clone()).unwrap();
            x=x.try_sub(self.clone().try_solve_approx(res)?).unwrap();
        }
        Ok(x)
    }
}