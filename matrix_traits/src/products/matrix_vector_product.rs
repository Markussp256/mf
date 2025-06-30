

use crate::{ColVector, ColVectorAnyConstruct, Matrix};

use super::vector_vector_product::AnyVectorVectorProduct;

pub trait MatrixVectorProduct<Rhs : ColVector> : Matrix {
    type Output : ColVector;
    fn matrix_vector_product(self, rhs:Rhs) -> Self::Output;
}

// impl code using only method from Matrix/Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn any_matrix_vector_product_impl
    <F3,
     Lhs    : Matrix<Row=LhsRow>,
     LhsRow : AnyVectorVectorProduct<Rhs,Output=F3> ,
     Rhs    : Clone+ColVector,
     Out    : ColVectorAnyConstruct<T=F3>>(lhs:Lhs,rhs:Rhs) -> Option<Out> {
    if lhs.ncols() != rhs.len() { return None; }
    Out::any_from_vec(
        lhs.into_rows()
           .map(|r|r.any_vector_vector_product(rhs.clone()).unwrap())
           .collect()).ok()
}

pub trait TryMatrixVectorProduct<Rhs : ColVector> : Matrix {
    type Output : ColVector;
    fn try_matrix_vector_product(self, rhs:Rhs) -> Option<Self::Output>;
}

pub trait AnyMatrixVectorProduct<Rhs : ColVector> : Matrix {
    type Output : ColVector;
    fn any_matrix_vector_product(self, rhs:Rhs) -> Option<Self::Output>;
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