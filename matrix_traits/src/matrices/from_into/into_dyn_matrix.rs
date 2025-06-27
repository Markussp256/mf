use crate::{MatrixDynamicallySized, IntoMatrix, FromMatrix, Matrix};

pub trait IntoDynMatrix : Matrix {
    type Output : MatrixDynamicallySized<T=Self::T>+FromMatrix<Self>;

    fn into_dyn_matrix(self) -> Self::Output {
        self.into_matrix()
    }
}