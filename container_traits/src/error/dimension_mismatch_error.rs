use super::*;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("Some other error regarding dimensions occured")]
pub struct OtherDimensionMismatchError;


#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum DimensionMismatchError<Index> {
    #[error(transparent)] EmptyContainer            (#[from] EmptyContainerError),
    #[error(transparent)] IndexOutOfBounds          (#[from] IndexOutOfBoundsError<Index>),
    #[error(transparent)] LowerBoundUpperBound      (#[from] LowerBoundUpperBoundError<Index>),
    #[error(transparent)] LenTooSmall               (#[from] LenTooSmallError),
    #[error(transparent)] LenNotEqualError          (#[from] LensNotEqualError),
    #[error(transparent)] LenNotEqualToRequiredLen  (#[from] LenNotEqualToRequiredLenError),
    #[error(transparent)] SizeTooSmall              (#[from] SizeTooSmallError<Index>),
    #[error(transparent)] SizeNotEqualToRequiredSize(#[from] SizeNotEqualToRequiredSizeError<Index>),
    #[error(transparent)] Other                     (#[from] OtherDimensionMismatchError)
}


#[test]
fn implements_display() {
    fn assert_display_impl<T: std::fmt::Display>() {}
    assert_display_impl::<DimensionMismatchError<(usize,usize)>>();
}