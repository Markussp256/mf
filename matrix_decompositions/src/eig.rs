// use crate::{DiagonalMatrix, NormalMatrix, StiefelMatrix};

// pub trait Eig : NormalMatrix {
//     // provided methods
//     fn eig<D:DiagonalMatrix<Output = Self::Output>>(self) -> (StiefelMatrix<Self>, D);
// }

// // currently Eig is implemented iff NormalMatrix
// impl<M:NormalMatrix> Eig for M {
//     fn eig<D:DiagonalMatrix<Output = Self::Output>>(self) -> (StiefelMatrix<Self>, D) {
//         (StiefelMatrix::try_new(self.clone()).unwrap(), D::try_new(self.clone()).unwrap()) 
//     }
// }