use super::DimensionMismatchError;


// we do not use #[from] because we want to implemt From more general

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum ContainerConstructError<Index> {
    #[error("dimension mismatch: {0}")]
    DimensionMismatch(DimensionMismatchError<Index>),
    #[error("data does not satisfy required properties of target container")]
    DataDoesNotSatisfyRequiredPropertiesOfTargetContainer,
}

impl<Index,E:Into<DimensionMismatchError<Index>>> From<E> for ContainerConstructError<Index> {
    fn from(value: E) -> Self {
        ContainerConstructError::DimensionMismatch(value.into())
    }
}