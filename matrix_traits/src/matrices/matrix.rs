use container_traits::{AnyFromIterator, Container, IntoIter, Map, TryIntoElement};
use num_traits::{Zero,One};
use algebra_traits::{TryScalarproduct, Conjugate, Distance, Scalar, Tolerance};
use crate::row_col::*;
use super::MatrixTryConstruct;
use utils::{iter::IntoExactSizeIterator, kronecker_delta::kron_delta};

type U2=(usize,usize);

// can be dynamic or static sized
pub trait Matrix : Container<U2> {
    type Row:RowVectorAnyConstruct<T=Self::T>;
    type Col:ColVectorAnyConstruct<T=Self::T>;

    // not possible because matrix is maybe not saved rowwise
    // fn rows(&self) -> impl Iterator<Item=&Self::Row>;

    // not possible because matrix is maybe not saved colwise
    // fn cols(&self) -> impl Iterator<Item=&Self::Col>;

    fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row>;
    fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col>;

    // provided methods

    fn nrows(&self) -> usize {
        self.size()
            .0
    }

    fn ncols(&self) -> usize {
        self.size()
            .1
    }

    fn matrix_dimensions(&self) -> (usize,usize) {
        (self.nrows(), self.ncols())
    }

    fn len(&self) -> usize {
        self.nrows() * self.ncols()
    }

    fn is_square(&self) -> bool {
        self.nrows() == self.ncols()
    }

    fn is_close_to(self, rhs:impl Matrix<T=Self::T>) -> bool
    where Self::T : Distance+Tolerance, 
         <Self::T as Distance>::DistT : PartialOrd {
        assert!(self.matrix_dimensions() == rhs.matrix_dimensions(),
               "matrices do not have the same dimensions and can therefore not be compared");
        self.into_iterator()
            .zip(rhs.into_iterator())
            .all(|(l,r)|l.is_close_to(r))
    }

    // not possible because matrix is maybe not saved rowwise
    // fn get_row(&self, i:usize) -> Option<&Self::Row>

    fn row(&self, i:usize) -> Option<Self::Row> where Self::T : Clone {
        (i < self.nrows()).then(||
            Self::Row::any_from_iter(None,(0..self.ncols()).map(|j|self.get((i,j)).unwrap().clone())).ok().unwrap())
    }

    // not possible because matrix is maybe not saved colwise
    // fn get_col(&self, j:usize) -> Option<&Self::Col>

    fn col(&self, j:usize) -> Option<Self::Col> where Self::T : Clone {
        (j < self.ncols()).then(||
        Self::Col::any_from_iter(None, (0..self.nrows()).map(|i|self.get((i,j)).unwrap().clone())).ok().unwrap())
    }

    fn diagonal(&self) -> impl ExactSizeIterator<Item=&Self::T> {
        let min=Ord::min(self.ncols(),self.nrows());
        (0..min).into_iter()
                .map(|i|self.get((i,i)).unwrap())
    }

    fn into_diagonal(self) -> impl ExactSizeIterator<Item=Self::T> {
        let min=Ord::min(self.ncols(), self.nrows());
        self.into_rows()
            .enumerate()
            .map(|(j,r)|r.try_into_element(j).unwrap())
            .take(min)
    }

    fn is_a_zero_matrix(&self) -> bool where Self::T : Zero {
        self.iter()
            .all(Zero::is_zero)
    }

    fn is_an_identity_matrix(&self) -> bool where Self::T : Zero+One+PartialEq {
        self.is_square()
        && 
        self.indexed_iter()
            .all(|((i,j),aij)| aij == &kron_delta(i,j)) 
    }
}

pub fn into_iterator_impl<M:Matrix>(m:M) -> impl ExactSizeIterator<Item=M::T> {
    let len=m.len();
       m.into_rows()
        .map(|r|r.into_iterator())
        .flatten()
        .into_exact_size_iter(len)
}

pub fn into_indexed_iterator_impl<M:Matrix>(m:M) -> impl ExactSizeIterator<Item=(U2,M::T)> {
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
     RowOut:RowVectorAnyConstruct<T=F2>>(m:M,f:impl Fn(F) -> F2) -> Out {
        Out::try_from_rows(
            m.into_rows()
                .map(|r|r.map(&f))
        ).unwrap()
}

pub trait AlgebraMatrix
    : Matrix
     +Conjugate {
    // fails if index out of bounds
    fn try_col_sc_prod(&self, j0:usize, j1:usize) -> Option<Self::T> where Self::T:Clone;
}

impl<F : Scalar,
     M : Matrix<T=F>
        +Conjugate> AlgebraMatrix for M where Self::Col : TryScalarproduct<TryScProdT = Self::T> {
            fn try_col_sc_prod(&self, j0:usize, j1:usize) -> Option<Self::T> where Self::T:Clone,  {
                let c0=self.col(j0)?;
                let c1=self.col(j1)?;
                c0.try_scalar_product(c1)
            }
        }