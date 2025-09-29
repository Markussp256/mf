pub mod row_col_vector;
pub use row_col_vector::*;

pub mod row_col_vector_view;
pub use row_col_vector_view::View;


// use crate::Transpose;

// pub trait BuildMatrix<Col:ColVector> : RowVector {
//     type Matrix:crate::Matrix;
// }

// pub trait BuildMatrixSquare : Transpose + BuildMatrix<<Self as Transpose>::Output>
//     where <Self as Transpose>::Output : ColVector {}


// impl<Row : RowVector+Transpose<Output=Col>+BuildMatrix<Col>,
//      Col : ColVector> BuildMatrixSquare for Row {}