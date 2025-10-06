
use std::ops::Neg;
use algebra_derive::{ClosedMul, ClosedTryDiv, ClosedTryMul, ClosedDiv};
use container_traits::{Get, IndexOutOfBoundsError, IntoIter, IntoIterIndexed, IsEmpty, ItemT, Iter, IterIndexed, NumberOfDegreesOfFreedom, OCTSize, Size, TryIntoElement};

use crate::matrix_shapes::{MatrixNotTall, MatrixNotWide};
use crate::{Matrix, MatrixView, MatrixSquare, MatrixConstructError, MatrixSquareTryConstruct};
use crate::matrices::from_into::{AsBaseMatrix, AsBaseSquareMatrix, IntoBaseMatrix, IntoBaseSquareMatrix};
use crate::matrix_operations::Det;

type U2=(usize,usize);

// we can not add or sub MatrixWithDet because its difficult to compute determinant of result

#[derive(Clone,Debug,
         ClosedMul,
         ClosedTryMul,
         ClosedDiv,
         ClosedTryDiv)]

pub struct MatrixWithDet<M:MatrixView> {
    m:M,
    det:M::T
}

impl<M:MatrixView> MatrixWithDet<M> {
    pub fn new(m:M, det:M::T) -> Self {
        Self{m,det}
    }

    pub fn matrix(&self) -> &M {
        &self.m
    }

    pub fn into_matrix(self) -> M {
        self.m
    }

    pub fn det(&self) -> &M::T {
        &self.det
    }

    pub fn from_matrix(m:M) -> Self where M : Clone+Det<DetF=M::T> {
        let det=m.clone().det();
        Self::new(m,det)
    }

    pub fn into_parts(self) -> (M,M::T) {
        (self.m, self.det)
    }
}

impl<M:MatrixView> Get<U2,M::T> for MatrixWithDet<M> {
    fn get(&self,index:U2) -> Result<&M::T,IndexOutOfBoundsError<U2>> {
        self.m.get(index)
    }
}

impl<M:MatrixSquare> Iter<M::T> for MatrixWithDet<M> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a M::T> where M::T:'a {
        self.m.iter()
    }
}

impl<M:MatrixSquare> IterIndexed<U2,M::T> for MatrixWithDet<M> {
    fn iter_indexed<'a>(&'a self) -> impl ExactSizeIterator<Item=(U2,&'a M::T)> where M::T:'a {
        self.m.iter_indexed()
    }
}

impl<M:MatrixSquare> ItemT for MatrixWithDet<M> {
    type T=M::T;
}

impl<M:Matrix+MatrixSquare> TryIntoElement<U2,M::T> for MatrixWithDet<M> {
    fn try_into_element(self,index:U2) -> Result<M::T,IndexOutOfBoundsError<U2>> {
        self.m
            .try_into_element(index)
    }
}

impl<M:Matrix+MatrixSquare> IntoIter<M::T> for MatrixWithDet<M> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=M::T> {
        self.m
            .into_iterator()
    }
}

impl<M:Matrix+MatrixSquare> IntoIterIndexed<U2,M::T> for MatrixWithDet<M> {
    fn into_iter_indexed(self) -> impl ExactSizeIterator<Item=(U2,M::T)> {
        self.m
            .into_iter_indexed()
    }
}

impl<M:MatrixSquare> Size<U2> for MatrixWithDet<M> {
    fn size(&self) -> U2 {
        self.m
            .size()
    }
}

impl<M:MatrixSquare+IsEmpty> IsEmpty for MatrixWithDet<M> {
    fn is_empty(&self) -> bool {
        self.m
            .is_empty()
    }
}

impl<M:MatrixSquare+OCTSize<U2>> OCTSize<U2> for MatrixWithDet<M> {
    const OCTSIZE:Option<U2> = M::OCTSIZE;
}

impl<M:MatrixSquare> NumberOfDegreesOfFreedom<M::T> for MatrixWithDet<M> {
    fn ndofs(&self) -> usize {
        self.m
            .ndofs()
    }
}


impl<M:MatrixSquare> MatrixView for MatrixWithDet<M> {
    
    type RowView<'a>=M::RowView<'a> where M : 'a;

    type ColView<'a>=M::ColView<'a> where M : 'a;

    fn nrows(&self) -> usize {
        self.m
            .nrows()
    }

    fn ncols(&self) -> usize {
        self.m
            .ncols()
    }
    
    fn try_row_view<'a>(&'a self, i:usize) -> Result<Self::RowView<'a>,IndexOutOfBoundsError<usize>> {
        self.m
            .try_row_view(i)
    }
    
    fn try_col_view<'a>(&'a self, j:usize) -> Result<Self::ColView<'a>,IndexOutOfBoundsError<usize>> {
        self.m
            .try_col_view(j)
    }
}

impl<M:Matrix+MatrixSquare> Matrix for MatrixWithDet<M> {
    
    type Row=M::Row;

    type Col=M::Col;

    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
        self.m
            .into_rows()
    }

    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
        self.m
            .into_cols()
    }
}

impl<M:MatrixSquare> IntoBaseSquareMatrix for MatrixWithDet<M> {
    type Output = M;
    fn into_base_square_matrix(self) -> M {
        self.m
    }
}

impl<M:MatrixSquare> AsBaseSquareMatrix for MatrixWithDet<M> {
    type Output=M;
    fn base_square_matrix(&self) -> &M {
        &self.m
    }
}

impl<M:MatrixSquare+IntoBaseMatrix> IntoBaseMatrix for MatrixWithDet<M> {
    type Output = <M as IntoBaseMatrix>::Output;
    fn into_base_matrix(self) -> <M as IntoBaseMatrix>::Output {
        self.m
            .into_base_matrix()
    }
}

impl<M:MatrixSquare+AsBaseMatrix> AsBaseMatrix for MatrixWithDet<M> {
    type Output=<M as AsBaseMatrix>::Output;
    fn base_matrix(&self) -> &<M as AsBaseMatrix>::Output {
        self.m
            .base_matrix()
    }
}

impl<M:MatrixSquare> MatrixNotTall for MatrixWithDet<M> {}
impl<M:MatrixSquare> MatrixNotWide for MatrixWithDet<M> {}
impl<M:MatrixSquare> MatrixSquare  for MatrixWithDet<M> {}

impl<F : Neg<Output=F>,
     M : MatrixSquare<T=F>> Neg for MatrixWithDet<M> where M : Neg<Output=M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let (m, det)=self.into_parts();
        // each sign flipping of a row changes sign
        // hence it depends if the number of rows is odd if the sign flips.
        let det=if m.n() % 2 == 0 { det } else { -det };
        Self::new(-m,det)
    }
}

impl<F : Clone+Neg<Output=F>,
     M : MatrixSquareTryConstruct<T=F>> MatrixWithDet<M> {
    pub fn try_neg_col(self,j:usize) -> Result<Self,MatrixConstructError> {
        let det=-self.det.clone();
        self.m
            .try_neg_col(j)
            .map(|m|Self::new(m,det))
    }

    pub fn try_neg_row(self,i:usize) -> Result<Self,MatrixConstructError> {
        let det=-self.det.clone();
        self.m
            .try_neg_row(i)
            .map(|m|Self::new(m,det))
    }
}