use crate::{MatrixTryConstruct, MatrixConstructError, Matrix};
use container_traits::AnyIntoContainer;

pub trait TryFromMatrix<M2:Matrix<T=Self::T>> : Matrix {
    fn try_from_matrix(m2:M2) -> Result<Self,MatrixConstructError>;
}

impl<F,M:MatrixTryConstruct<T=F>, M2:Matrix<T=F>> TryFromMatrix<M2> for M {
    fn try_from_matrix(m2:M2) -> Result<Self,MatrixConstructError> {
        let f=|(i,j)|m2.get((i,j)).unwrap();
        Self::try_accept(m2.matrix_dimensions(), f)?;
        Self::try_from_rows(m2.into_rows()
                                    .map(|r|r.any_into_container().ok().unwrap()))
    }
}

pub trait TryIntoMatrix<M2:Matrix<T=Self::T>> : Matrix {
    fn try_into_matrix(self) -> Result<M2,MatrixConstructError>;
}

impl<F,M:Matrix<T=F>,M2:TryFromMatrix<M,T=F>> TryIntoMatrix<M2> for M {
    fn try_into_matrix(self) -> Result<M2,MatrixConstructError> {
        M2::try_from_matrix(self)
    }
}