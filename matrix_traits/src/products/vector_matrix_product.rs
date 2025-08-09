use crate::{RowVector, RowVectorTryConstruct, Matrix};
use super::vector_vector_product::TryVectorVectorProduct;

pub trait VectorMatrixProduct<Rhs : Matrix> : RowVector {
    type Output : RowVector;
    fn vector_matrix_product(self, rhs:Rhs) -> <Self as VectorMatrixProduct<Rhs>>::Output;
}

// impl code using only method from Matrix/Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_vector_matrix_product_impl
    <F3,
     Lhs    : Clone+RowVector+TryVectorVectorProduct<Rhs::Col,Output=F3>,
     Rhs    : Matrix,
     Out    : RowVectorTryConstruct<T=F3>>(lhs:Lhs,rhs:Rhs) -> Option<Out> {
    if lhs.len() != rhs.nrows() { return None; }
    Out::any_from_iter(
        None,
        rhs.into_cols()
                .map(|r|lhs.clone().try_vector_vector_product(r).unwrap())).ok()
}

pub trait TryVectorMatrixProduct<Rhs : Matrix> : RowVector {
    type Output : RowVector;
    fn try_vector_matrix_product(self, rhs:Rhs) -> Option<<Self as TryVectorMatrixProduct<Rhs>>::Output>;
}