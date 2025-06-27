use container_traits::{AnyFromIterator, IntoIter};
use crate::{MatrixConstruct, Matrix};

pub trait FromMatrix<M2:Matrix<T=Self::T>> : Matrix {
    fn from_matrix(m2:M2) -> Self;
}

impl<F,
     M  : MatrixConstruct<T=F>,
     M2 : Matrix<T=F>> FromMatrix<M2> for M {
    fn from_matrix(m2:M2) -> Self {
        Self::try_from_rows(
            m2.into_rows()
              .map(|r|M::Row::any_from_iter(None, r.into_iterator()).unwrap())).ok().unwrap()
    }
}

pub trait IntoMatrix<M2:Matrix<T=Self::T>> : Matrix {
    fn into_matrix(self) -> M2;
}

impl<F, M:Matrix<T=F>, M2:FromMatrix<M,T=F>> IntoMatrix<M2> for M {
    fn into_matrix(self) -> M2 {
        M2::from_matrix(self)
    }
}