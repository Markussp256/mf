use algebra::VectorDyn;
use container_traits::{ChangeT, Empty, IndexOutOfBoundsError, IntoIter, IterMut, Len, LenNotEqualToRequiredLenError, OneElement, Pop, Push, TryInsert, TryRemove, Zeros};

use either::Either;
use matrix_traits::{try_matrix_matrix_product_impl, try_matrix_vector_product_impl, ColVectorAnyConstruct, FromMatrix, Matrix, MatrixConstruct, MatrixDynamic, MatrixTryConstruct, RowVectorAnyConstruct, IntoDynMatrix, TryMatrixMatrixProduct, TryMatrixVectorProduct};
use utils::{iter::IntoExactSizeIterator, IntoThis};

use std::ops::Mul;
use num_traits::Zero;

use crate::{MatrixColDyn, MatrixRowDyn};
use super::MatrixGeneric;

pub type MatrixDyn<F>=MatrixGeneric<MatrixRowDyn<F>,MatrixColDyn<F>>;

type U2=(usize,usize);


impl<F:'static> Zeros<U2,F> for MatrixDyn<F> {
    fn zeros(size:U2) -> Self where F:Zero {
        let r=||MatrixRowDyn::<F>::zeros(size.1);
        let iter=
            std::iter::repeat_with(r)
                .into_exact_size_iter(size.0);
        Self::try_from_rows(iter).unwrap()
    }
}

impl<F:'static> OneElement<F> for MatrixDyn<F> {
    fn one_element(t:F) -> Self {
        Self::try_from_rows(std::iter::once(MatrixRowDyn::one_element(t))).unwrap()
    }
}

impl<F:'static> MatrixConstruct for MatrixDyn<F> {}

// impl<F:'static> MatrixDynamic   for MatrixDyn<F> {
//     fn try_push_row(&mut self, row:Self::Row) -> Result<(),Self::Row> {
//         if self.is_empty() || row.len() == self.ncols() {
//             self.as_mut()
//                 .push(row);
//             Ok(())
//         } else {
//             Err(row)
//         }
//     }

//     fn try_push_col(&mut self, col:Self::Col) -> Result<(),Self::Col> {
//         if self.is_empty() || col.len() == self.ncrows() {
//             self.as_mut()
//                 .iter_mut()
//                 .zip(col.into_iterator())
//                 .for_each(|(r,rn)|r.push(rn));
//             Ok(())
//         } else {
//             Err(col)
//         }
//     }

//     fn try_pop_row(&mut self) -> Option<Self::Row> {
//         self.as_mut()
//             .pop()
//     }

//     fn try_pop_col(&mut self) -> Option<Self::Col> {
//         (self.ncols() > 0).then(||
//             Self::Col::try_from_iter(
//                 InstanceStructureDescriptor::Size(self.nrows()), 
//                 self.as_mut()
//                     .iter_mut()
//                     .map(|r|r.pop().unwrap())).unwrap())
//     }

//     fn try_remove_row(& mut self, index:usize) -> Result<Self::Row, IndexOutOfBoundsError<usize>> {
//         self.as_mut()
//             .try_remove(index)
//     }

//     fn try_remove_col(& mut self, index:usize) -> Result<Self::Col, IndexOutOfBoundsError<usize>> {
//         IndexOutOfBoundsError::try_new(&self.ncols(),&index)?;
//         Ok(Self::Col::try_from_iter(
//             InstanceStructureDescriptor::Size(self.nrows()), 
//                 self.as_mut()
//                     .iter_mut()
//                     .map(|r|r.try_remove(index).unwrap())).unwrap())
//     }

//     fn try_insert_row(& mut self, row:Self::Row, index:usize) -> Result<(), Either<LenNotEqualToRequiredLenError, IndexOutOfBoundsError<usize>>> {
//         if !self.is_empty() {
//             LenNotEqualToRequiredLenError::try_new(self.ncols(),row.len())
//                 .map_err(Either::Left)?;
//         }
//         IndexOutOfBoundsError::try_new(&self.nrows(),&index)
//             .map_err(Either::Right)?;
//         self.as_mut()
//             .try_insert(index, row);
//         Ok(())
//     }

//     fn try_insert_col(& mut self, col:Self::Col, index:usize) -> Result<(), Either<LenNotEqualToRequiredLenError, IndexOutOfBoundsError<usize>>> {
//         if !self.is_empty() {
//             LenNotEqualToRequiredLenError::try_new(self.nrows(),col.len())
//                 .map_err(Either::Left)?;
//         }
//         IndexOutOfBoundsError::try_new(&self.ncols(),&index)
//             .map_err(Either::Right)?;
//         self.as_mut()
//             .iter_mut()
//             .zip(col.into_iterator())
//             .for_each(|(r,ri)|r.try_insert(index,ri));
//         Ok(())
//     }
// }


impl<F:'static> From<MatrixColDyn<F>> for MatrixDyn<F> {
    fn from(value: MatrixColDyn<F>) -> Self {
        let mut m=Self::empty();
        assert!(m.try_push_col(value).is_ok());
        m
    }
}

impl<F:'static> From<VectorDyn<F>> for MatrixDyn<F> {
    fn from(value: VectorDyn<F>) -> Self {
        value.into_this::<MatrixColDyn<F>>()
             .into()
    }
}

impl<F:'static> MatrixDynamicallySized for MatrixDyn<F> {}

impl<F:'static> MatrixDynamic for MatrixDyn<F> {
    fn try_push_row(&mut self, row:Self::Row) -> Result<(),Self::Row> {
        if self.is_empty() || row.len() == self.ncols() {
            self.as_mut()
                .push(row);
            Ok(())
        } else {
            Err(row)
        }
    }

    fn try_push_col(&mut self, col:Self::Col) -> Result<(),Self::Col> {
        if self.is_empty() {
            // we push rows of length 1
            col.into_iterator()
               .for_each(|ci|self.try_push_row(Self::Row::from([ci])).ok().unwrap());
            Ok(())
        } else if col.len() == self.nrows() {
            self.as_mut()
                .iter_mut()
                .zip(col.into_iterator())
                .for_each(|(ri,ci)|ri.push(ci));
            Ok(())
        } else {
            Err(col)
        }
    }

    fn try_pop_row(&mut self) -> Option<Self::Row> {
        self.as_mut()
            .pop()
    }

    fn try_pop_col(&mut self) -> Option<Self::Col> {
        (self.ncols() > 0).then(||
            Self::Col::from_iter(
                self.as_mut()
                    .iter_mut()
                    .map(|r|r.pop().unwrap())))
    }
    
    fn try_remove_row(& mut self, index:usize) -> Result<Self::Row, IndexOutOfBoundsError<usize>> {
        self.as_mut()
            .try_remove(index)
            .ok_or_else(||IndexOutOfBoundsError::new(&self.nrows(), &index))
    }
    
    fn try_remove_col(& mut self, index:usize) -> Result<Self::Col, IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.ncols(),&index)?;
        Ok(Self::Col::from_iter(
            self.as_mut()
                .iter_mut()
                .map(|r|r.try_remove(index).unwrap())
        ))
    }
    
    fn try_insert_row(& mut self, row:Self::Row, index:usize) -> Result<(), Either<LenNotEqualToRequiredLenError, IndexOutOfBoundsError<usize>>> {
        if !self.is_empty() {
            LenNotEqualToRequiredLenError::try_new(self.ncols(), row.len())
                .map_err(Either::Left)?
        }
        self.as_mut()
            .try_insert(index,row)
            .ok_or_else(||Either::Right(IndexOutOfBoundsError::new(&self.nrows(),&index)))
    }
    
    fn try_insert_col(& mut self, col:Self::Col, index:usize) -> Result<(), Either<LenNotEqualToRequiredLenError, IndexOutOfBoundsError<usize>>> {
        if !self.is_empty() {
            LenNotEqualToRequiredLenError::try_new(self.nrows(), col.len())
                .map_err(Either::Left)?
        }
        IndexOutOfBoundsError::try_new(&self.ncols(),&index)
            .map_err(Either::Right)?;
        self.as_mut()
            .iter_mut()
            .zip(col.into_iterator())
            .for_each(|(r,ci)|r.try_insert(index,ci).unwrap());
        Ok(())
    }
}

macro_rules! try_matrix_vector_impl {
    ($tr:ident $(, $fn:ident)?) => {
        impl<F:'static+Mul<F2,Output=F3>,F2:Clone,F3:'static+Zero> $tr<MatrixColDyn<F2>> for MatrixDyn<F> {
            $(type Output = MatrixColDyn<F3>;
            fn $fn(self, rhs:MatrixColDyn<F2>) -> Option<MatrixColDyn<F3>> {
                any_matrix_vector_product_impl(self,rhs)
            })?
        }
    };
}
try_matrix_vector_impl!(TryMatrixVectorProduct, try_matrix_vector_product);

macro_rules! try_matrix_matrix_impl {
    ($tr:ident $(, $fn:ident)?) => {
        impl<F:'static+Clone+Mul<F2,Output=F3>,F2:'static+Clone,F3:'static+Zero> $tr<MatrixDyn<F2>> for MatrixDyn<F> {
            $(type Output = MatrixDyn<F3>;
            fn $fn(self, rhs:MatrixDyn<F2>) -> Option<MatrixDyn<F3>> {
                any_matrix_matrix_product_impl(self,rhs)
            })?
        }
    };
}
try_matrix_matrix_impl!(TryMatrixMatrixProduct, try_matrix_matrix_product);



impl<F:'static,const M:usize,const N:usize> From<super::Matrix<F,M,N>> for MatrixDyn<F> {
    fn from(value: super::Matrix<F,M,N>) -> Self {
        <Self as FromMatrix<_>>::from_matrix(value)
    }
}

impl<F    :'static,
     Row  : RowVectorAnyConstruct<T=F>,
     Col  : ColVectorAnyConstruct<T=F>+ChangeT<Row,Output=C>,
     C    :'static+ColVectorAnyConstruct<T=Row>> IntoDynMatrix for MatrixGeneric<Row,Col> {
        type Output=MatrixDyn<F>;
     }


// #[test]
// fn test_matrix_matrix_mul<F:Mul<V,Output=V>,V:Zero>(q:MatrixDyn<F>,r:MatrixDyn<V>) -> Option<MatrixDyn<V>> {
//     use matrix_traits::AnyMatrixMatrixProduct;
//     q.any_matrix_matrix_product(r)
// }


matrix_decompositions::impl_qr_tall!(MatrixDyn<V>, MatrixDyn<F>, MatrixDyn<V>, RightTriangular);
matrix_decompositions::impl_qr_non_tall!(Wide<MatrixDyn<V>>,  SquareMatrixDyn<F>, Wide<MatrixDyn<V>>, RightTriangular);
matrix_decompositions::impl_qr_non_tall!(SquareMatrixDyn<V>, SquareMatrixDyn<F>, SquareMatrixDyn<V>, SquareRightTriangular);
     