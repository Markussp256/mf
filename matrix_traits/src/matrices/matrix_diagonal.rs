use container_traits::{TryFromVec, LinearContainerConstructError};
use num_traits::{Zero,One};

use super::{MatrixViewSquare, MatrixSquareTryConstruct};

use crate::error::MatrixConstructError;

// fn is_diagonal<F2:Zero>(&self) -> bool where Self::F: AsRef<F2> {
//     let nrows=self.nrows();
//     let ncols=self.ncols();
//     if nrows != ncols {
//         return false;
//     }
//     for i in 0..nrows {
//         for j in 0..ncols {
//             let e:&F2=self.get(i,j).unwrap().as_ref();
//             if i != j && !e.is_zero() {
//                 return false;
//             }
//         }
//     }
//     true
// }

pub trait MatrixDiagonal : MatrixViewSquare where Self::T : Zero {}



pub trait MatrixDiagonalTryConstruct : MatrixDiagonal
                                     + MatrixSquareTryConstruct
                                     + Sized
                                     + TryFromVec<Self::T,LinearContainerConstructError>
    where Self::T : Zero {

    // provided

    fn try_zero(len:usize) -> Result<Self,MatrixConstructError> {
        let vs:Vec<Self::T>=std::iter::repeat_with(
            <Self::T as Zero>::zero)
                .take(len)
                .collect();
        Self::try_from_vec(vs)
            .map_err(|e|e.into())
    }

    fn try_identity(len:usize) -> Result<Self,MatrixConstructError> where Self::T:One {
        let vs:Vec<Self::T>=std::iter::repeat_with(
            <Self::T as One>::one)
                .take(len)
                .collect();
        Self::try_from_vec(vs)
            .map_err(|e|e.into())
    }
}
impl<F:Zero, M:MatrixDiagonal<T=F>+MatrixSquareTryConstruct+Sized+TryFromVec<F,LinearContainerConstructError>> MatrixDiagonalTryConstruct for M {}