use container_traits::{AnyFromIterator, LinearContainerConstructError};

use crate::{ColVectorView, Matrix, MatrixView, RowVector, RowVectorTryConstruct, RowVectorView};
use super::vector_vector_product::{TryVectorVectorProduct,TryIntoVectorVectorProduct};

pub trait VectorMatrixProduct<Rhs : MatrixView> : RowVectorView {
    type Output : RowVectorView;
    fn vector_matrix_product(&self, rhs:&Rhs) -> <Self as VectorMatrixProduct<Rhs>>::Output;
}


pub trait IntoVectorMatrixProduct<Rhs : Matrix> : RowVector {
    type Output : RowVectorView;
    fn into_vector_matrix_product(&self, rhs:Rhs) -> <Self as IntoVectorMatrixProduct<Rhs>>::Output;
}


// impl code using only method from Matrix/Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_into_vector_matrix_product_impl
    <F3,
     Lhs    : Clone+RowVector+TryIntoVectorVectorProduct<Rhs::Col,Output=F3>,
     Rhs    : Matrix,
     Out    : RowVectorTryConstruct<T=F3>>(lhs:&Lhs,rhs:Rhs) -> Option<Out> {
    if lhs.len() != rhs.nrows() { return None; }
    Out::any_from_iter(
        None,
        rhs.into_cols()
                .map(|r|lhs.clone().try_into_vector_vector_product(r).unwrap())).ok()
}

pub fn try_vector_matrix_product_impl
    <F2 : Clone,
     F3,
     Lhs    : RowVector+TryVectorVectorProduct<Rhs::ColView,Output=F3>,
     Rhs    : MatrixView<T=F2, ColView = RhsColView>,
     Out    : RowVectorTryConstruct<T=F3>>(lhs:&Lhs,rhs:&Rhs) -> Option<Out> {
    if lhs.len() != rhs.nrows() { return None; }
    Out::any_from_iter(
        None,
            (0..rhs.ncols())
                    .into_iter()
                    .map(|j|rhs.col_view(j).unwrap())
                    .map(|rhs_col|lhs.try_vector_vector_product(&rhs_col).unwrap())).ok()
}


pub trait TryVectorMatrixProduct<Rhs : MatrixView> : RowVectorView {
    type Output : RowVectorView;
    fn try_vector_matrix_product(&self, rhs:&Rhs) -> Option<<Self as TryVectorMatrixProduct<Rhs>>::Output>;
}

pub trait TryIntoVectorMatrixProduct<Rhs : Matrix> : RowVector {
    type Output : RowVectorView;
    fn try_into_vector_matrix_product(self, rhs:Rhs) -> Option<<Self as TryIntoVectorMatrixProduct<Rhs>>::Output>;
}