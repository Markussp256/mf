use crate::{MatrixTryConstruct,MatrixConstructError,Matrix};
use container_traits::{AnyFromIterator, Iter};

pub trait TrySubMatrix : Matrix {
    fn try_submatrix<M2:MatrixTryConstruct<T=Self::T>>(
        &self,
        r_inds:impl ExactSizeIterator<Item=usize>,
        c_inds:impl ExactSizeIterator<Item=usize>) -> Result<M2,MatrixConstructError> where Self::T:Clone {
        let (nrows,ncols)=self.matrix_dimensions();
        let r_inds:Vec<usize>=r_inds.into_iter().collect();
        let c_inds:Vec<usize>=c_inds.into_iter().collect();
        if r_inds.iter().any(|ri|ri >= &nrows) { return Err(MatrixConstructError::DimensionMismatch); }
        if c_inds.iter().any(|ci|ci >= &ncols) { return Err(MatrixConstructError::DimensionMismatch); }
        let row=|ri|
            M2::Row::any_from_iter(
                None,
                c_inds.iter()
                            .cloned()
                            .map(|ci|self.get((ri,ci)).unwrap().clone()));
        r_inds.into_iter()
              .map(row)
              .collect::<Result<Vec<M2::Row>,_>>()
              .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)
              .and_then(|v|M2::try_from_rows(v.into_iter()))
    }
}
impl<M:Matrix> TrySubMatrix for M {}