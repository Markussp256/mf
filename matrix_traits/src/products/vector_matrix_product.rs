
use crate::{MatrixView, RowVector, RowVectorTryConstruct, RowVectorView, VectorCanNotBeMultipliedWithMatrixError, VectorConstructError};
use super::vector_vector_product::TryVectorVectorProduct;

pub trait VectorMatrixProduct<Rhs : MatrixView> : RowVectorView {
    type Output : RowVectorView;
    fn vector_matrix_product(&self, rhs:&Rhs) -> <Self as VectorMatrixProduct<Rhs>>::Output;
}


// impl code using only method from Matrix/Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_vector_matrix_product_impl
    <'a,
     Lhs    : RowVector+TryVectorVectorProduct<Rhs::ColView<'a>,Output=Out::T>,
     Rhs    : 'a+MatrixView,
     Out    : RowVectorTryConstruct>(lhs:&'a Lhs,rhs:&'a Rhs) -> Result<Out,VectorConstructError> {
    VectorCanNotBeMultipliedWithMatrixError::try_new(lhs.len(),rhs.nrows())?;
    Out::any_from_iter(
        None,
        rhs.col_views()
                 .map(|rhs_col|lhs.try_vector_vector_product(&rhs_col).unwrap()))
            .map_err(|e|e.into())
}


pub trait TryVectorMatrixProduct<Rhs : MatrixView> : RowVectorView {
    type Output : RowVector;
    fn try_vector_matrix_product(&self, rhs:&Rhs) -> Result<<Self as TryVectorMatrixProduct<Rhs>>::Output,VectorConstructError>;
}