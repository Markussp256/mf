use std::ops::Neg;
use crate::Matrix;
use crate::error::MatrixConstructError;

use algebra_traits::ClosedNeg;
use container_traits::{TryCommonLength, AnyFromIterator, TryMap, CommonLengthError, ContainerTryConstruct, Get, IntoIter, Iter};
use utils::iter::{ChainExactSize, IntoExactSizeIterator};

type U2=(usize,usize);

pub trait MatrixTryConstruct : Matrix + ContainerTryConstruct<U2,MatrixConstructError> {

    // required methods

    fn try_from_rows(rows:impl ExactSizeIterator<Item=Self::Row>) -> Result<Self,MatrixConstructError>;

    // provided methods

    fn try_accept_fn_returns_owned((nrows,ncols):U2,f:impl Fn(U2) -> Self::T) -> Result<(),MatrixConstructError> {
        let vals:Vec<Vec<Self::T>>=
            (0..nrows).map(|i|(0..ncols).map(|j|f((i,j))).collect())
                      .collect();
        Self::try_accept((nrows, ncols), |(i,j)|&vals[i][j])
    }

    fn try_accept_vec_of_rows<'a>(rows:impl ExactSizeIterator<Item= &'a Self::Row>) -> Result<U2,MatrixConstructError> where Self : 'a {
        let rows:Vec<&Self::Row>=rows.collect();
        let (nrows,ncols)=match Self::Row::try_common_length(rows.iter().cloned()) {
            Ok(sz) => sz,
            Err(CommonLengthError::EmptyVec(_)) => (0,0),
            Err(CommonLengthError::NotAllHaveSameLength(vs)) => { return Err(MatrixConstructError::RowsDoNotHaveTheSameLength(vs)); },
        };
        Self::try_accept((nrows, ncols), |(i,j)|rows[i].get(j).unwrap())?;
        Ok((nrows,ncols))
    }

    fn try_accept_vec_of_cols<'a>(cols:impl ExactSizeIterator<Item=&'a Self::Col>) -> Result<U2,MatrixConstructError> where Self : 'a {
        let cols:Vec<&Self::Col>=cols.collect();
        let (ncols,nrows)=match Self::Col::try_common_length(cols.iter().cloned()) {
            Ok(sz) => sz,
            Err(CommonLengthError::EmptyVec(_)) => (0,0),
            Err(CommonLengthError::NotAllHaveSameLength(vs)) => {return Err(MatrixConstructError::ColsDoNotHaveTheSameLength(vs));},
        };
        Self::try_accept((nrows, ncols), |(i,j)|cols[j].get(i).unwrap())?;
        Ok((nrows,ncols))
    }


    fn try_from_cols(cols:impl ExactSizeIterator<Item=Self::Col>) -> Result<Self,MatrixConstructError> {
        let cols:Vec<Self::Col>=cols.collect();
        let (nrows,_)=Self::try_accept_vec_of_cols(cols.iter())?;
        let mut iters:Vec<_>=
            cols.into_iter()
                .map(|c|c.into_iterator())
                .collect();
        Self::try_from_rows(std::iter::repeat_with(||
            Self::Row::any_from_iter(None,iters.iter_mut().map(|iter|iter.next().unwrap())).unwrap())
                .into_exact_size_iter(nrows))
    }

    // fn try_from_fn(nrows:usize, ncols:usize, f:impl FnU2 -> Self::T) -> Result<Self,MatrixConstructError> {
    //     Self::try_accept_fn_returns_owned(nrows, ncols, &f)?;
    //     let orows:Vec<Self::Row>=
    //         (0..nrows).into_iter()
    //                   .map(|i|Self::Row::any_from_iter(None,(0..ncols).map(|j|f(i,j))).ok().unwrap())
    //                   .collect();
    //     Self::try_from_rows(orows)
    // }


    // zeros/identity should be implemented for concrete type so that TryZeros/TryIdentity not necessary 
    // fn try_zeros(nrows:usize, ncols:usize) -> Result<Self,MatrixConstructError> where Self::T : Zero {
    //     Self::Row::any_zeros(ncols)?;
    //     Self::try_from_rows(
    //         std::iter::repeat_with(
    //             ||Self::Row::any_zeros(ncols).unwrap())
    //                                                    .take(nrows)
    //                                                    .collect())
    // }

    // fn try_identity(n:usize) -> Result<Self,MatrixConstructError> where Self::T : Zero+One {
    //     Self::any_from_fn((n,n),|(i,j)|kron_delta(i, j))
    // }

    fn try_neg_row(self,i:usize) -> Result<Self,MatrixConstructError> where Self::T : ClosedNeg {
        if i >= self.nrows() { return Err(MatrixConstructError::DimensionMismatch); }
        let neg_row=|r:Self::Row|r.try_map(Neg::neg).ok().unwrap();
        Self::try_from_rows(self.into_rows()
                                .enumerate()
                                .map(|(ii,r)|if ii == i { neg_row(r) } else { r }))
    }

    fn try_neg_col(self,j:usize) -> Result<Self,MatrixConstructError> where Self::T : ClosedNeg {
        if j >= self.ncols() { return Err(MatrixConstructError::DimensionMismatch); }
        let neg_col=|c:Self::Col|c.try_map(Neg::neg).ok().unwrap();
        Self::try_from_cols(self.into_cols()
                                .enumerate()
                                .map(|(jj,c)|if jj == j { neg_col(c) } else { c }))
    }

    fn try_concat_vertically(self, rhs:Self) -> Result<Self,MatrixConstructError> {
        if self.ncols() == rhs.ncols() {
            Self::try_from_rows(
                self.into_rows()
                    .chain_exact_size(rhs.into_rows()))
        } else {
            Err(MatrixConstructError::DimensionMismatch)
        }
    }

    fn try_concat_horizontally(self, rhs:Self) -> Result<Self,MatrixConstructError> {
        if self.nrows() == rhs.nrows() {
            Self::try_from_cols(
                self.into_cols()
                    .chain_exact_size(rhs.into_cols()))
        } else {
            Err(MatrixConstructError::DimensionMismatch)
        }
    }
}



// pub trait TryFromSuperMatrix<M2:Matrix> : TryFromSuperContainer<U2, M2, MatrixConstructError> {
//     fn try_from_super(m:M2, start:U2, size:U2) -> Result<Self, MatrixConstructError> {
//         <Self as TryFromSuperContainer<U2,M2,MatrixConstructError>>::try_from_super(m, start, size)
//     }
// }