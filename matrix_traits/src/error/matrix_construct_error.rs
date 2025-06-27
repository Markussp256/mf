use container_traits::ContainerConstructError;

type U2=(usize,usize);

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum MatrixConstructError {
    #[error("provided rows do not all have the same length")]
    RowsDoNotHaveTheSameLength(Vec<usize>),
    #[error("provided columns do not all have the same length")]
    ColsDoNotHaveTheSameLength(Vec<usize>),
    #[error("dimension mismatch")]
    DimensionMismatch,
    #[error("data does not satisfy required properties of matrix type")]
    DataDoesNotSatisfyRequiredPropertiesOfMatrixType
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

impl<E:Into<ContainerConstructError<U2>>> From<E> for MatrixConstructError {
    fn from(e:E) -> Self {
        let e: ContainerConstructError<U2>=e.into();
        match e {
            ContainerConstructError::DimensionMismatch(_)
            => MatrixConstructError::DimensionMismatch,
            ContainerConstructError::DataDoesNotSatisfyRequiredPropertiesOfTargetContainer
            => MatrixConstructError::DataDoesNotSatisfyRequiredPropertiesOfMatrixType,
        }
    }
}
