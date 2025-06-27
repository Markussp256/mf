
use std::ops::Mul;
use container_traits::ChangeT;
use num_traits::Zero;

use crate::{RowVector, RowVectorAnyConstruct, Matrix, ColVector};
use super::vector_vector_product::{TryVectorVectorProduct,AnyVectorVectorProduct};

pub trait VectorMatrixProduct<Rhs> {
    type Output;
    fn vector_matrix_product(self, rhs:Rhs) -> Self::Output;
}

// impl code using only method from Matrix/Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_vector_matrix_product_impl
    <F3,
     Lhs    : Clone+RowVector+AnyVectorVectorProduct<Rhs::Col,Output=F3>,
     Rhs    : Matrix,
     Out    : RowVectorAnyConstruct<T=F3>>(lhs:Lhs,rhs:Rhs) -> Option<Out> {
    if lhs.len() != rhs.nrows() { return None; }
    Out::any_from_iter(
        None,
        rhs.into_cols()
                .map(|r|lhs.clone().any_vector_vector_product(r).unwrap())).ok()
}

pub trait TryVectorMatrixProduct<Rhs:Matrix> : RowVector {
    type Output : RowVectorAnyConstruct;
    fn try_vector_matrix_product(self, rhs:Rhs) -> Option<<Self as TryVectorMatrixProduct<Rhs>>::Output>;
}

impl<F   : Mul<M::T,Output=F3>,
     F3  : Zero,
     Row : Clone+RowVector<T=F>+TryVectorVectorProduct<Col,Output=F3>+ChangeT<F3,Output=RowF3>,
     RowF3 : RowVectorAnyConstruct<T=F3>,
     M   : Matrix<T=F,Col=Col>,
     Col : Clone+ColVector<T=F> > TryVectorMatrixProduct<M> for Row {
    type Output=RowF3;
    fn try_vector_matrix_product(self, rhs:M) -> Option<RowF3> {
         try_vector_matrix_product_impl(self,rhs)
    }
}