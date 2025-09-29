pub mod eig_def;
pub use eig_def::EigStruct;

mod eig_impl;

// use crate::{DiagonalMatrix, MatrixNormal, StiefelMatrix};

// pub trait Eig : MatrixNormal {
//     // provided methods
//     fn eig<D:DiagonalMatrix<Output = Self::Output>>(self) -> (StiefelMatrix<Self>, D);
// }

// // currently Eig is implemented iff MatrixNormal
// impl<M:MatrixNormal> Eig for M {
//     fn eig<D:DiagonalMatrix<Output = Self::Output>>(self) -> (StiefelMatrix<Self>, D) {
//         (StiefelMatrix::try_new(self.clone()).unwrap(), D::try_new(self.clone()).unwrap()) 
//     }
// }