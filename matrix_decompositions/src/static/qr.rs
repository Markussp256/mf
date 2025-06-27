
use std::ops::Mul;

use algebra_traits::{Det, Scalar, TryDiv, Vectorspace1d};

use crate::Vector;
use crate::matrix::{Matrix, MatrixDyn, QRDyn, SpecialStiefelMatrix, SquareMatrixDyn, SquareQRDyn};



use utils::{IntoThis, TryIntoThis};

#[derive(Clone, Debug, derive_more::Into)]
pub struct QR<F, const M:usize, const N:usize>(QRDyn<F>);



impl<F:Clone+Scalar, const M:usize, const N:usize> TryFrom<Matrix<F,M,N>> for QR<F,M,N> {
    type Error=Matrix<F, M, N>;
    fn try_from(m:Matrix<F,M,N>) -> Result<Self, Self::Error> {
        match m.clone()
               .into_this::<MatrixDyn<F>>()
               .try_into_this::<QRDyn<F>>() {
            Ok(qrdyn) => Ok(Self(qrdyn)),
            Err(_) => Err(m)
        }
    }
}

impl<F:Clone+Scalar, const M:usize, const N:usize> QR<F,M,N> {
    pub fn solve_least_squares<V:Clone+TryDiv<Output=F>+Vectorspace1d>(&self, b:Vector<V,M>) -> Vector<V,N> where F:Mul<V, Output=V> {
        self.0
            .try_solve_least_squares(b.into()).unwrap()
            .try_into()
            .ok()
            .unwrap()
    }
}



#[derive(Clone, Debug, derive_more::Into)]
pub struct SquareQR<F:Scalar, const N:usize>(SquareQRDyn<F>);


impl<F:Clone+Scalar, const N:usize> SquareQR<F, N> {
    pub fn q(&self) -> SpecialStiefelMatrix<F, N> {
        self.0
            .q()
            .clone()
            .try_into()
            .ok().unwrap()
    }

    pub fn r(&self) -> Matrix<F, N, N> {
        self.0
            .r()
            .matrix()
            .clone()
            .try_into()
            .ok().unwrap()
    }
}

impl<F:Clone+Scalar, const N:usize> From<Matrix<F,N,N>> for SquareQR<F,N> {
    fn from(m:Matrix<F,N,N>) -> Self {
        Self(SquareQRDyn::<F>::from(m.into_this::<SquareMatrixDyn<F>>()))
    }
}

impl<F:Clone+Scalar, const N:usize> Det for SquareQR<F,N> {
    type Output=F;
    fn det(self) -> F {
        self.0
            .det()
    }
}


// square system
impl<F:Clone+Scalar, const N:usize> SquareQR<F,N> {

    pub fn try_into_inverse_matrix(self) -> Option<Matrix<F, N, N>> {
        self.0
            .try_into_inverse_matrix()
            .map(|m|m.try_into().ok().unwrap())
    }

    pub fn try_solve<V:Clone+TryDiv<Output=F>+Vectorspace1d>(&self, b:Vector<V,N>) -> Option<Vector<V,N>> where F:Mul<V, Output=V> {
        self.0
            .try_solve(b.into())?
            .try_into().ok()
    }

}

