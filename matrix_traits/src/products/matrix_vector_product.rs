

use container_traits::{AnyFromIterator, LinearContainerConstructError};

use crate::{error::{MatrixCanNotBeMultipliedWithVectorError, VectorConstructError}, ColVector, ColVectorTryConstruct, ColVectorView, Matrix, MatrixView};

use super::vector_vector_product::TryVectorVectorProduct;

pub trait MatrixVectorProduct<Rhs : ColVectorView> : MatrixView {
    type Output : ColVectorView;
    fn matrix_vector_product(&self, rhs:&Rhs) -> Self::Output;
}


pub trait IntoMatrixVectorProduct<Rhs : ColVector> : Matrix {
    type Output : ColVectorView;
    fn into_matrix_vector_product(self, rhs:Rhs) -> Self::Output;
}

// impl code using only method from Matrix/Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_matrix_vector_product_impl
    <'a,
     F : Clone,
     F3,
     Lhs    : MatrixView<T=F,RowView<'a>=LhsRow>,
     LhsRow : AnyFromIterator<F,LinearContainerConstructError>+TryVectorVectorProduct<Rhs,Output=F3> ,
     Rhs    : ColVectorView,
     Out    : ColVectorTryConstruct<T=F3>>(lhs:&'a Lhs,rhs:&'a Rhs) -> Result<Out,VectorConstructError> {
    MatrixCanNotBeMultipliedWithVectorError::try_new(lhs.ncols(),rhs.len())?;
    Out::any_from_iter(
        None,
        lhs.row_views()
                 .map(|r|r.try_vector_vector_product(rhs).unwrap()))
        .map_err(|e|e.into())
}


pub fn try_into_matrix_vector_product_impl
    <F3,
     Lhs    : Matrix<Row=LhsRow>,
     LhsRow : TryVectorVectorProduct<Rhs,Output=F3> ,
     Rhs    : ColVectorView,
     Out    : ColVectorTryConstruct<T=F3>>(lhs:Lhs,rhs:&Rhs) -> Result<Out,VectorConstructError> {
    MatrixCanNotBeMultipliedWithVectorError::try_new(lhs.ncols(),rhs.len())?;
    Out::any_from_iter(None,
        lhs.into_rows()
           .map(|r|r.try_vector_vector_product(rhs).unwrap())
           )
        .map_err(|e|e.into())
}



pub trait TryMatrixVectorProduct<Rhs : ColVectorView> : MatrixView {
    type Output : ColVectorView;
    fn try_matrix_vector_product(&self, rhs:&Rhs) -> Result<Self::Output,VectorConstructError>;
}

pub trait TryIntoMatrixVectorProduct<Rhs : ColVector> : Matrix {
    type Output : ColVectorView;
    fn try_into_matrix_vector_product(self, rhs:Rhs) -> Result<Self::Output,VectorConstructError>;
}


// impl<M:Matrix<Row=Row,Col=Col>,
//      F3,
//      Row:VectorVectorProduct<Col2,Output=F3>,
//      Col:ChangeT<F3,Output=ColOut>,
//      Col2: Clone+ColVector,
//      ColOut:ColVectorTryConstruct<T=F3>> MatrixVectorProduct<Col2> for M {
//     type Output=ColOut;
//     fn matrix_vector_product(self, rhs:Col2) -> Self::Output {
//         Self::Output::try_from_iter(
//         self.into_rows()
//             .map(|row|row.vector_vector_product(rhs.clone()))).ok().unwrap()
//     }
// }

// impl<M:Matrix<Row=Row,Col=Col>,
//      F3,
//      Row:TryVectorVectorProduct<Col2,Output=F3>,
//      Col:ChangeT<F3,Output=ColOut>,
//      Col2: Clone+ColVector,
//      ColOut:ColVectorTryConstruct<T=F3>> TryMatrixVectorProduct<Col2> for M {
//     type Output=ColOut;
//     fn try_matrix_vector_product(self, rhs:Col2) -> Option<Self::Output> {
//         if self.ncols() != rhs.len() {
//             return None;
//         }
//         Self::Output::try_from_iter(
//         self.into_rows()
//             .map(|row|row.try_vector_vector_product(rhs.clone()).unwrap())).ok()
//     }
// }