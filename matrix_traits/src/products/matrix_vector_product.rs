
use std::ops::Mul;
use container_traits::ChangeT;
use num_traits::Zero;

use crate::{ColVector, ColVectorAnyConstruct, Matrix, RowVector};

use super::vector_vector_product::AnyVectorVectorProduct;

pub trait MatrixVectorProduct<Rhs> {
    type Output;
    fn matrix_vector_product(self, rhs:Rhs) -> Self::Output;
}

// impl code using only method from Matrix/Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_matrix_vector_product_impl
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

pub trait TryMatrixVectorProduct<Rhs> {
    type Output;
    fn try_matrix_vector_product(self, rhs:Rhs) -> Option<Self::Output>;
}


impl<F,
     F3  : Zero,
     M   : Matrix<T=F,Row=Row,Col=Col>,
     Row : RowVector<T=F>+AnyVectorVectorProduct<Rhs,Output=F3>,
     Col : ColVector<T=F>+ChangeT<F3,Output=ColF3>,
     ColF3 : ColVectorAnyConstruct<T=F3>,
     Rhs : Clone+ColVector> TryMatrixVectorProduct<Rhs> for M
     where F:Mul<Rhs::T,Output=F3> {
    type Output = ColF3;
    fn try_matrix_vector_product(self, rhs:Rhs) -> Option<ColF3> {
        let lhs_dims=self.matrix_dimensions();
        
        let res: Option<ColF3>=try_matrix_vector_product_impl(self,rhs);
        if let Some(r)=&res {
            let out_len=r.len();
            assert_eq!(out_len, lhs_dims.0);
        }
        res
    }
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