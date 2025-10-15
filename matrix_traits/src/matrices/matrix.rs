use container_traits::{AnyFromIterator, Container, IndexOutOfBoundsError, IntoIter, Map, TryIntoElement};
use algebra_traits::{Distance, Tolerance};
use crate::row_col::*;
use super::MatrixTryConstruct;
use utils::iter::IntoExactSizeIterator;
use super::MatrixView;
type U2=(usize,usize);

// can be dynamic or static sized
pub trait Matrix : MatrixView + Container<U2> {
    type Row:RowVectorTryConstruct<T=Self::T>;
    type Col:ColVectorTryConstruct<T=Self::T>;

    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row>;
    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col>;

    // provided methods

    fn into_is_close_to(self, rhs:impl Matrix<T=Self::T>) -> bool
    where Self::T : Distance+Tolerance, 
         <Self::T as Distance>::DistT : PartialOrd {
        assert_eq!(self.matrix_dimensions(), rhs.matrix_dimensions());
        self.into_iterator()
            .zip(rhs.into_iterator())
            .all(|(l,r)|l.is_close_to(r))
    }

    // not possible because matrix is maybe not saved rowwise
    // fn get_row(&self, i:usize) -> Option<&Self::Row>

    fn try_row(&self, i:usize) -> Result<Self::Row, IndexOutOfBoundsError<usize>> where Self::T : Clone {
        IndexOutOfBoundsError::try_new(&self.nrows(),&i)?;
        Ok(Self::Row::any_from_iter(None,(0..self.ncols()).map(|j|self.get((i,j)).unwrap().clone())).ok().unwrap())
    }

    fn rows(&self) -> impl ExactSizeIterator<Item=Self::Row> where Self::T : Clone {
        (0..self.nrows())
            .map(|i|self.try_row(i).unwrap())
    }

    // not possible because matrix is maybe not saved colwise
    // fn get_col(&self, j:usize) -> Option<&Self::Col>

    fn try_col(&self, j:usize) -> Result<Self::Col,IndexOutOfBoundsError<usize>> where Self::T : Clone {
        IndexOutOfBoundsError::try_new(&self.ncols(),&j)?;
        Ok(Self::Col::any_from_iter(None, (0..self.nrows()).map(|i|self.get((i,j)).unwrap().clone())).ok().unwrap())
    }

    fn cols(&self) -> impl ExactSizeIterator<Item=Self::Col> where Self::T : Clone {
        (0..self.ncols())
            .map(|j|self.try_col(j).unwrap())
    }

    fn into_diagonal(self) -> impl ExactSizeIterator<Item=Self::T> {
        let min=Ord::min(self.ncols(), self.nrows());
        self.into_rows()
            .enumerate()
            .map(|(j,r)|r.try_into_element(j).unwrap())
            .take(min)
    }
}

pub fn into_iterator_impl<M:Matrix>(m:M) -> impl ExactSizeIterator<Item=M::T> {
    let len=m.len();
       m.into_rows()
        .map(|r|r.into_iterator())
        .flatten()
        .into_exact_size_iter(len)
}

pub fn into_iter_indexedator_impl<M:Matrix>(m:M) -> impl ExactSizeIterator<Item=(U2,M::T)> {
    let len=m.len();
    m.into_rows()
     .enumerate()
     .map(|(i,r)|r.into_iterator().enumerate().map(move |(j,aij)|((i,j),aij)))
     .flatten()
     .into_exact_size_iter(len)
}


pub fn impl_map
    <F,
     M:Matrix<T=F,Row=Row>,
     Row : RowVector<T=F>+Map<F,F2,Output=RowOut>, 
     F2,
     Out:MatrixTryConstruct<T=F2,Row=RowOut>,
     RowOut:RowVectorTryConstruct<T=F2>>(m:M,f:impl Fn(F) -> F2) -> Out {
        Out::try_from_rows(
            m.into_rows()
                .map(|r|r.map(&f))
        ).unwrap()
}
