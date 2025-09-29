use crate::{Matrix, MatrixConstructError, MatrixTryConstruct, TryPopCol, TryPopRow};
use container_traits::{AnyFromIterator, IndexOutOfBoundsError, IntoIter, Iter, LinearContainerConstructError};
use utils::iter::{IntoExactSizeIterator, VarStep};

// to be merged with matrix_operations::try_into_sub

pub trait TryIntoSubMatrixDyn : Matrix {
    fn try_into_sub_matrix<M2:MatrixTryConstruct<T=Self::T>>(
        self,
        r_inds:impl ExactSizeIterator<Item=usize>,
        c_inds:impl ExactSizeIterator<Item=usize>) -> Result<M2,MatrixConstructError> {
        let (nrows,ncols)=self.matrix_dimensions();
        let r_inds:Vec<usize>=r_inds.into_iter().collect();
        let c_inds:Vec<usize>=c_inds.into_iter().collect();
        if r_inds.len() == 0 {
            return M2::try_from_rows(std::iter::empty());
        }
        if c_inds.len() == 0 {
            return M2::try_from_cols(std::iter::empty());
        }
        let inds_2_steps=|inds:Vec<usize>|{
            let mut inds=inds;
            inds.sort();
            ((0..inds.len()-1)
                .into_iter()
                .map(|i|
                     if i == 0 {
                         inds[i]
                     } else {
                         inds[i]-inds[i-1]-1
                     }).collect::<Vec<usize>>(),inds.last().unwrap().clone())
        };
        let (r_skips,r_max):(Vec<usize>,usize)=inds_2_steps(r_inds);
        let (c_skips,c_max):(Vec<usize>,usize)=inds_2_steps(c_inds);
        IndexOutOfBoundsError::try_new(&(nrows,ncols),&(r_max,c_max))?;
        let get_sub_row=|row:Self::Row|
        {
            M2::Row::any_from_iter(
                None,
                VarStep::new(row.into_iterator(),c_skips.iter().cloned()))
        };
        let rows:Vec<M2::Row>=VarStep::new(self.into_rows(),r_skips.into_iter())
            .map(|r|get_sub_row(r))
            .collect::<Result::<Vec<M2::Row>,LinearContainerConstructError>>()
            .map_err(|_|MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType)?;
        M2::try_from_rows(rows.into_iter())
    }

    fn try_into_top_left<M2:MatrixTryConstruct<T=Self::T>>(self, (nrows,ncols):(usize,usize)) -> Result<M2,MatrixConstructError> where Self::T:Clone {
        self.try_into_sub_matrix(0..nrows,0..ncols)
    }
}
impl<M:Matrix> TryIntoSubMatrixDyn for M {}


fn counter_without(n: usize, skip: usize) -> impl ExactSizeIterator<Item = usize> {
    (0..n).filter(move |&x| x != skip)
          .into_exact_size_iter(n-1)
}

// remove one column and one row
pub trait TryIntoSubMatrix : Matrix {
    type Output : MatrixTryConstruct<T=Self::T>;

    fn try_into_sub_matrix(self,(i,j):(usize,usize)) -> Result<Self::Output,MatrixConstructError> {
        let (nrows,ncols)=self.matrix_dimensions();
        IndexOutOfBoundsError::try_new(&(nrows,ncols),&(i,j))?;
        <Self as TryIntoSubMatrixDyn>::try_into_sub_matrix::<Self::Output>(
            self,
            counter_without(nrows, i),
            counter_without(ncols, j))
    }
}

impl<T,
     M  : TryPopRow<T=T,Output=M2>,
     M2 : TryPopCol<T=T,Output=M3>,
     M3 : MatrixTryConstruct<T=T>> TryIntoSubMatrix for M {
        type Output = M3;
}