
#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("The provided Length ({provided_len}) is not equal to the required length ({required_len})")]
pub struct LenNotEqualToRequiredLenError {
    required_len:usize,
    provided_len:usize
}

impl LenNotEqualToRequiredLenError {
    pub fn try_new(required_len:usize,provided_len:usize) -> Result<(),Self> {
        if provided_len == required_len {
            Ok(())
        } else {
            Err(Self{required_len,provided_len})
        }
    }

    // panics if it is not error
    pub fn new(required_len:usize,provided_len:usize) -> Self {
        Self::try_new(required_len, provided_len)
            .err()
            .unwrap()
    }
}


#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("The provided size ({provided_size}) is not equal to the required size ({required_size})")]
pub struct SizeNotEqualToRequiredSizeError<Index> {
    required_size:Index,
    provided_size:Index
}

impl<Index:Clone+PartialEq> SizeNotEqualToRequiredSizeError<Index> {
    pub fn try_new(required_size:Index,provided_size:Index) -> Result<(),Self> {
        if provided_size == required_size {
            Ok(())
        } else {
            Err(Self{required_size,provided_size})
        }
    }

    pub fn try_new_ref(required_size:&Index,provided_size:&Index) -> Result<(),Self> {
        if provided_size == required_size {
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

impl From<LenNotEqualToRequiredLenError> for SizeNotEqualToRequiredSizeError<usize> {
    fn from(value: LenNotEqualToRequiredLenError) -> Self {
        SizeNotEqualToRequiredSizeError{required_size:value.required_len,provided_size:value.provided_len}
    }
}

impl From<SizeNotEqualToRequiredSizeError<usize>> for LenNotEqualToRequiredLenError {
    fn from(value: SizeNotEqualToRequiredSizeError<usize>) -> Self {
        LenNotEqualToRequiredLenError{required_len:value.required_size,provided_len:value.provided_size}
    }
}