

use crate::{error::{MatrixCanNotBeMultipliedWithVectorError, VectorConstructError}, ColVector, ColVectorTryConstruct, ColVectorView, Matrix, MatrixView};

use super::vector_vector_product::TryVectorVectorProduct;

pub trait MatrixVectorProduct<Rhs : ColVectorView> : MatrixView {
    type Output : ColVector;
    
    fn into_matrix_vector_product(self, rhs:&Rhs) -> Self::Output;
    fn matrix_vector_product(&self, rhs:&Rhs) -> Self::Output where Self : Clone {
        self.clone()
            .into_matrix_vector_product(rhs)
    }
}

// impl code using only method from Matrix/Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_matrix_vector_product_impl
    <'a,
     Lhs    : MatrixView<RowView<'a>=LhsRow>,
     LhsRow : TryVectorVectorProduct<Rhs,Output=Out::T> ,
     Rhs    : ColVectorView+Clone,
     Out    : ColVectorTryConstruct>(lhs:&'a Lhs,rhs:&'a Rhs) -> Result<Out,VectorConstructError> {
    MatrixCanNotBeMultipliedWithVectorError::try_new(lhs.ncols(),rhs.len())?;
    Out::any_from_iter(
        None,
        lhs.row_views()
                 .map(|r|r.try_into_vector_vector_product(rhs.clone()).unwrap()))
        .map_err(|e|e.into())
}


pub fn try_into_matrix_vector_product_impl
    <Lhs    : Matrix<Row=LhsRow>,
     LhsRow : TryVectorVectorProduct<Rhs,Output=Out::T> ,
     Rhs    : ColVectorView+Clone,
     Out    : ColVectorTryConstruct>(lhs:Lhs,rhs:&Rhs) -> Result<Out,VectorConstructError> {
    MatrixCanNotBeMultipliedWithVectorError::try_new(lhs.ncols(),rhs.len())?;
    Out::any_from_iter(
        None,
        lhs.into_rows()
           .map(|r|r.try_into_vector_vector_product(rhs.clone()).unwrap())
           )
        .map_err(|e|e.into())
}



pub trait TryMatrixVectorProduct<Rhs : ColVectorView> : MatrixView {
    type Output : ColVector;
    
    fn try_into_matrix_vector_product(self, rhs:&Rhs) -> Result<Self::Output,VectorConstructError>;

    fn try_matrix_vector_product(&self, rhs:&Rhs) -> Result<Self::Output,VectorConstructError> where Self : Clone {
        self.clone()
            .try_into_matrix_vector_product(rhs)
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