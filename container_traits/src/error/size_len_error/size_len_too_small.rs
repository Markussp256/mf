use crate::ContainerIndex;

// for example if iterator has not enough elements to construct the container
#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("The provided Length ({provided_len}) is smaller than the required length ({required_len})")]
pub struct LenTooSmallError {
    required_len:usize,
    provided_len:usize
}

impl LenTooSmallError {
    pub fn try_new(required_len:usize,provided_len:usize) -> Result<(),Self> {
        if provided_len < required_len {
            Err(Self{required_len,provided_len})
        } else {
            Ok(())
        }
    }

    // panics if it is not error
    pub fn new(required_len:usize,provided_len:usize) -> Self {
        Self::try_new(required_len, provided_len)
            .err()
            .unwrap()
    }
}


// this error is similar as indexoutofbounds but the focus is more on the size relative to index than index relative to size 

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("The provided size ({provided_size}) is not elementwise larger equal than the required size ({required_size})")]
pub struct SizeTooSmallError<Index> {
    required_size:Index,
    provided_size:Index
}

impl<Index:ContainerIndex> SizeTooSmallError<Index> {

    pub fn try_new(required_size:Index,provided_size:Index) -> Result<(),Self> {
        if provided_size.is_elem_wise_larger_eq(&required_size) {
            Ok(())
        } else {
            Err(Self{required_size,provided_size})
        }
    }

    pub fn try_new_ref(required_size:&Index,provided_size:&Index) -> Result<(),Self> {
        if provided_size.is_elem_wise_larger_eq(required_size) {
            Ok(())
        } else {
            Err(Self{required_size:required_size.clone(),provided_size:provided_size.clone()})
        }
    }

    // panics if it is not error
    pub fn new(required_size:Index,provided_size:Index) -> Self {
        Self::try_new(required_size, provided_size)
            .err()
            .unwrap()
    }
}

impl From<LenTooSmallError> for SizeTooSmallError<usize> {
    fn from(value: LenTooSmallError) -> Self {
        SizeTooSmallError{
            required_size:value.required_len,
            provided_size:value.provided_len}
    }
}

impl From<SizeTooSmallError<usize>> for LenTooSmallError {
    fn from(value: SizeTooSmallError<usize>) -> Self {
        LenTooSmallError{
            required_len:value.required_size,
            provided_len:value.provided_size}
    }
}