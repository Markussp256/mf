use container_traits::error::*;
use crate::error::MatricesCanNotBeMultipliedError;
use utils::from_via;

type U2=(usize,usize);

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("matrix is square, number of rows {nrows} must be the same as number of columns {ncols}")]
pub struct MatrixSquareError{nrows:usize,ncols:usize}

impl MatrixSquareError {
    pub fn try_new(nrows:usize,ncols:usize) -> Result<(),Self> {
        if nrows == ncols {
            Ok(())
        } else {
            Err(Self{nrows,ncols})
        }
    }

    pub fn new(nrows:usize,ncols:usize) -> Self {
        Self::try_new(nrows,ncols)
            .unwrap_err()
    }
}


#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum MatrixConstructError {
    #[error("provided rows do not all have the same length {0:?}")]
    RowsDoNotHaveTheSameLength(Vec<usize>),
    #[error("provided columns do not all have the same length {0:?}")]
    ColsDoNotHaveTheSameLength(Vec<usize>),
    #[error(transparent)]
    DimensionMismatchUSize(#[from] DimensionMismatchError<usize>),
    #[error(transparent)]
    DimensionMismatchU2(#[from] DimensionMismatchError<U2>),
    #[error(transparent)]
    MatricesCanNotBeMultiplied(#[from] MatricesCanNotBeMultipliedError),
    #[error(transparent)]
    MatrixViewSquare(#[from] MatrixSquareError),
    #[error("data does not satisfy required properties of matrix type")]
    DataDoesNotSatisfyRequiredPropertiesOfMatrixType
}


macro_rules! impl_from_cce {
    ($index:ty, $enum_item:ident) => {
        impl From<ContainerConstructError<$index>> for MatrixConstructError {
            fn from(e:ContainerConstructError<$index>) -> Self {
                match e {
                    ContainerConstructError::DimensionMismatch(ee)
                    => MatrixConstructError::$enum_item(ee),
                    ContainerConstructError::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer
                    => MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType,
                }
            }
        }
    };
}
impl_from_cce!(usize, DimensionMismatchUSize);
impl_from_cce!((usize,usize), DimensionMismatchU2);

macro_rules! from_via_both {
    ($name:ident) => {
        from_via!(impl From<$name<usize>> for MatrixConstructError, via DimensionMismatchError<usize>);
        from_via!(impl From<$name<U2>>    for MatrixConstructError, via DimensionMismatchError<U2>);
    };
}
from_via_both!(IndexOutOfBoundsError);
from_via_both!(LowerBoundUpperBoundError);
from_via_both!(SizeTooSmallError);
from_via_both!(SizeNotEqualToRequiredSizeError);

from_via!(impl From<EmptyContainerError>           for MatrixConstructError, via DimensionMismatchError<U2>);
from_via!(impl From<LenTooSmallError>              for MatrixConstructError, via DimensionMismatchError<usize>);
from_via!(impl From<LensNotEqualError>             for MatrixConstructError, via DimensionMismatchError<usize>);
from_via!(impl From<LenNotEqualToRequiredLenError> for MatrixConstructError, via DimensionMismatchError<usize>);
from_via!(impl From<OtherDimensionMismatchError>   for MatrixConstructError, via DimensionMismatchError<U2>);
