use super::*;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("Some other error regarding dimensions occured")]
pub struct OtherDimensionMismatchError;


#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum DimensionMismatchError<Index> {
    IndexOutOfBounds          (#[from] IndexOutOfBoundsError<Index>),
    LenTooSmall               (#[from] LenTooSmallError),
    LenNotEqualError          (#[from] LensNotEqualError),
    LenNotEqualToRequiredLen  (#[from] LenNotEqualToRequiredLenError),
    SizeTooSmall              (#[from] SizeTooSmallError<Index>),
    SizeNotEqualToRequiredSize(#[from] SizeNotEqualToRequiredSizeError<Index>),
    Other                     (#[from] OtherDimensionMismatchError)
}