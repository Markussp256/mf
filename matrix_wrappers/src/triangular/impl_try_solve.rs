use std::ops::{Mul, Neg, Sub};
use num_traits::Zero;
use algebra_traits::{ClosedTrySub, Scalar, TryDiv, TryInv, TrySolve, TrySolveHomogeneous};
use container_traits::{ChangeT, Get, IntoInner, IntoIter, IntoSum, LensNotEqualError, LinearContainerTryConstruct, TryFromFn};
use matrix_traits::*;

use super::RightTriangular;

// since this can fail in any case we only implement try


trait TrySolveApprox<Rhs=Self> {
    type Output;
    fn try_solve_approx(self, b:Rhs) -> Result<Self::Output, MatrixSolveError>;
}

impl<F   : Clone+Zero+Mul<F2,Output=F3>,
     F2  : Clone+Zero,
     F3  : Clone+TryDiv<F,Output=F2>+Zero+Sub<Output=F3>,
     M   : Matrix<T=F>,
     Rhs : LinearContainerTryConstruct<T=F3>+ChangeT<F2,Output=Out>,
     Out : LinearContainerTryConstruct<T=F2>> TrySolveApprox<Rhs> for RightTriangular<M> where Self : Matrix<T=F> {
    type Output = Out;

    fn try_solve_approx(self, b:Rhs) -> Result<Self::Output, MatrixSolveError> {
        if !self.is_square() { return Err(MatrixSolveError::MatrixNotSquare); }
        let n=self.nrows();
        LensNotEqualError::try_new(n, b.len())?;
        let mut x:Vec<F2>=std::iter::repeat_with(F2::zero).take(n).collect();

        // because
        let vs:Vec<(M::Row,F3)>=
            self.into_inner()
                .into_rows()
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
        Ok(Out::try_from_vec(x).unwrap())
    }
}

impl<T   : Zero+Mul<Out::T,Output=Rhs::T>,
     M   : Matrix<T=T>,
     Rhs : Clone+ClosedTrySub+ColVectorTryConstruct,
     Out : Clone+ClosedTrySub+ColVector>
        TrySolve<Rhs,MatrixSolveError> for RightTriangular<M>
     where  Self : Sized+TrySolveApprox<Rhs,Output=Out>
                  +TryMatrixVectorProduct<Out,Output=Rhs>
                  +Matrix<T=T>
                  +Clone {
    type Output=Out;
    fn try_solve(self, rhs:Rhs) -> Result<Out, MatrixSolveError> {
        let mut x=self.clone().try_solve_approx(rhs.clone())?;
        for _ in 0..5 {
            let res:Rhs=self.clone().try_matrix_vector_product(x.clone()).unwrap()
                            .try_sub(rhs.clone()).ok().unwrap();
            x=x.try_sub(self.clone().try_solve_approx(res)?).ok().unwrap();
        }
        Ok(x)
    }
}

trait TrySolveInverseApprox : Sized {
    fn try_solve_inverse_approx(self) -> Result<Self, MatrixNotRegularError>;
}

impl<F   : Clone+Scalar,
     M   : Clone+MatrixSquareTryConstruct<T=F>+TryPopRow<Output=MR>,
     MR  : Matrix<T=F,Col=Col>+TryPopCol<Output=MRC>+TryPushRow<Output=M>,
     Col : ColVectorTryConstruct<T=F>+Neg<Output=Col>+Mul<F,Output=Col>,
     MRC : Matrix<T=F,Col=Col>+Clone+TryPushCol<Output=MR>> TrySolveInverseApprox for RightTriangular<M>
     where Self : Sized+Matrix<T=F>,
     RightTriangular<MRC> : Matrix<T=F,Col=Col>+TrySolveInverseApprox+TrySolveApprox<Col,Output=Col> {
    fn try_solve_inverse_approx(self) -> Result<Self, MatrixNotRegularError> {
        // base case
        let n=self.n();
        if n == 1 {
            let s00:F=self.get((0,0)).unwrap().clone();
            if s00.is_zero() {
                return Err(MatrixNotRegularError);
            }
            let s00i=F::one().try_div(s00).unwrap();
            return Ok(Self::try_from_fn((1,1),|(_,_)|s00i.clone()).unwrap());
        }
        let (mr,r)=self.try_pop_row().unwrap();
        let rnn=r.into_iterator().last().unwrap();
        if rnn.is_zero() {
            return Err(MatrixNotRegularError);
        }
        let rnni=F::one().try_div(rnn).unwrap();        
        let (mrc,c)=mr.try_pop_col().unwrap();
        let mrci=mrc.clone().try_solve_inverse_approx()?;
        let ci=mrc.try_solve_approx(-c * rnni.clone()).unwrap();
        let mri=mrci.try_push_col(ci).ok().unwrap();
        let ri=MR::Row::try_from_fn(n, |i| if i == n-1 { rnni.clone() } else { F::zero() }).unwrap();
        Ok(mri.try_push_row(ri).ok().unwrap())
    }
}


impl<F : Zero+Mul<Output=F>,
     M : Clone+Matrix<T=F>+TryMatrixMatrixProduct<Output=M>> TryInv for RightTriangular<M>
    where  Self : Sized+Matrix<T=F>+TrySolveInverseApprox {
    type Output=Self;
    type Error=MatrixNotRegularError;
    fn is_invertible(&self) -> Result<(),   Self::Error> {
        self.clone()
            .try_inv()
            .map(|_|())
    }

    fn try_inv(self) -> Result<Self, MatrixNotRegularError> {
        let mut x=self.clone().try_solve_inverse_approx()?;
        let mul=|a:&Self,b:&Self|a.clone().try_matrix_matrix_product(b.clone()).unwrap();
        for _ in 0..5 {
            x=mul(&x,&mul(&self,&x).try_solve_inverse_approx()?);
        }
        Ok(x)
    }
}


impl<T    : Clone+Scalar,
     M    : Matrix<T=T,Row=Row>+IntoDynMatrix<Output=MD>,
     Row  : RowVector<T=T>+Transpose<Output=Col>,
     Col  : Clone+ColVectorTryConstruct<T=T>,
     MD   : MatrixTryConstruct<T=T,Col=ColD>+TryPopCol<Output=MD>,
     ColD : ColVector<T=T>> TrySolveHomogeneous<MatrixSolveError> for RightTriangular<M>
     where RightTriangular<MD> : TrySolve<ColD,MatrixSolveError,Output=ColD> {
    type Output=Col;
    fn try_solve_homogeneous(self) -> Result<Col, MatrixSolveError> {
        let nrows=self.nrows();
        let on=
            self.diagonal()
                .enumerate()
                .find(|(_,v)|v.is_zero());
        if on.is_none() {
            return Err(MatrixSolveError::MatrixRegular(MatrixRegularError));
        }
        let n=on.unwrap().0;
        let md:RightTriangular<MD>=self.try_into_top_left((n,n+1)).unwrap();
        let (md,b)=md.try_pop_col().unwrap();
        let x:ColD=md.try_solve(b).ok().unwrap();
        Ok(Col::any_from_iter(
            None,
            x.into_iterator()
             .chain(std::iter::once(-T::one()))
             .chain(std::iter::repeat(T::zero()).take(nrows-(n+1)))
        ).ok().unwrap())
    }
}