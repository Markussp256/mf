
use crate::{Matrix, MatrixView, RowVector, RowVectorTryConstruct, RowVectorView, VectorCanNotBeMultipliedWithMatrixError, VectorConstructError};
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
     Out    : RowVectorTryConstruct<T=F3>>(lhs:&Lhs,rhs:Rhs) -> Result<Out,VectorConstructError> {
    VectorCanNotBeMultipliedWithMatrixError::try_new(lhs.len(),rhs.nrows())?;
    Out::any_from_iter(
        None,
        rhs.into_cols()
                .map(|r|lhs.clone().try_into_vector_vector_product(r).unwrap()))
            .map_err(|e|e.into())
}

pub fn try_vector_matrix_product_impl
    <'a,
     F2 : Clone,
     F3,
     Lhs    : RowVector+TryVectorVectorProduct<Rhs::ColView<'a>,Output=F3>,
     Rhs    : 'a+MatrixView<T=F2>,
     Out    : RowVectorTryConstruct<T=F3>>(lhs:&'a Lhs,rhs:&'a Rhs) -> Result<Out,VectorConstructError> {
    VectorCanNotBeMultipliedWithMatrixError::try_new(lhs.len(),rhs.nrows())?;
    Out::any_from_iter(
        None,
        rhs.col_views()
                 .map(|rhs_col|lhs.try_vector_vector_product(&rhs_col).unwrap()))
            .map_err(|e|e.into())
}


pub trait TryVectorMatrixProduct<Rhs : MatrixView> : RowVectorView {
    type Output : RowVectorView;
    fn try_vector_matrix_product(&self, rhs:&Rhs) -> Option<<Self as TryVectorMatrixProduct<Rhs>>::Output>;
}

pub trait TryIntoVectorMatrixProduct<Rhs : Matrix> : RowVector {
    type Output : RowVectorView;
    fn try_into_vector_matrix_product(self, rhs:Rhs) -> Option<<Self as TryIntoVectorMatrixProduct<Rhs>>::Output>;
}