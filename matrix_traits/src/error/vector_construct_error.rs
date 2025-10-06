use container_traits::error::*;
use utils::from_via;

type U2=(usize,usize);

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("Matrix with {0} columns can not be muliplied with vector of length {1}")]
pub struct MatrixCanNotBeMultipliedWithVectorError(usize,usize);

impl MatrixCanNotBeMultipliedWithVectorError {
    pub fn try_new(ncols:usize, len:usize) -> Result<(),Self> {
        if ncols == len {
            Ok(())
        } else {
            Err(Self(ncols,len))
        }
    }

    pub fn new(ncols:usize, len:usize) -> Self {
        Self::try_new(ncols,len)
            .unwrap_err()
    }
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("Vector of length {0} can not be muliplied with matrix with {1} rows")]
pub struct VectorCanNotBeMultipliedWithMatrixError(usize,usize);

impl VectorCanNotBeMultipliedWithMatrixError {
    pub fn try_new(len:usize, nrows:usize) -> Result<(),Self> {
        if len == nrows {
            Ok(())
        } else {
            Err(Self(len, nrows))
        }
    }

    pub fn new(len:usize, nrows:usize) -> Self {
        Self::try_new(len,nrows)
            .unwrap_err()
    }
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum VectorConstructError {
    #[error(transparent)]
    DimensionMismatch(#[from] DimensionMismatchError<usize>),
    #[error(transparent)]
    MatrixCanNotBeMultipliedWithVector(#[from] MatrixCanNotBeMultipliedWithVectorError),
    #[error(transparent)]
    VectorCanNotBeMultipliedWithMatrix(#[from] VectorCanNotBeMultipliedWithMatrixError),
    #[error("data does not satisfy required properties of vector type")]
    DataDoesNotSatisfyRequiredPropertiesOfVectorType
}

// impl From<ContainerConstructError<U2>> for MatrixConstructError {
//     fn from(value: ContainerConstructError<U2>) -> Self {
//         match value {
//             ContainerConstructError::DimensionMismatch(_)
//             => MatrixConstructError::DimensionMismatch,
//             ContainerConstructError::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer
//             => MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType,
//         }
//     }
// }

impl From<ContainerConstructError<usize>> for VectorConstructError {
    fn from(e:ContainerConstructError<usize>) -> Self {
        match e {
            ContainerConstructError::DimensionMismatch(e)
            => VectorConstructError::DimensionMismatch(e),
            ContainerConstructError::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer
            => VectorConstructError::DataDoesNotSatisfyRequiredPropertiesOfVectorType,
        }
    }
}

macro_rules! impl_from_via {
    ($name:ident $(<$index:ident>)?) => {
        from_via!(impl From<$name $(<$index>)?> for VectorConstructError, via DimensionMismatchError<usize>);
    };
}



impl_from_via!(IndexOutOfBoundsError          <usize>);
impl_from_via!(LowerBoundUpperBoundError      <usize>);
impl_from_via!(SizeTooSmallError              <usize>);
impl_from_via!(SizeNotEqualToRequiredSizeError<usize>);
impl_from_via!(EmptyContainerError          );
impl_from_via!(LenTooSmallError             );
impl_from_via!(LensNotEqualError            );
impl_from_via!(LenNotEqualToRequiredLenError);
impl_from_via!(OtherDimensionMismatchError  );
