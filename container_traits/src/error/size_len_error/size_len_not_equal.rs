
#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("The first length ({len0}) is not equal to the second length ({len1})")]
pub struct LensNotEqualError {
    len0:usize,
    len1:usize
}

impl LensNotEqualError {
    pub fn try_new(len0:usize,len1:usize) -> Result<(),Self> {
        if len0 == len1 {
            Ok(())
        } else {
            Err(Self{len0,len1})
        }
    }

    // panics if it is not error
    pub fn new(len0:usize, len1:usize) -> Self {
        Self::try_new(len0, len1)
            .err()
            .unwrap()
    }
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("The first size ({size0}) is not equal to the second size ({size1})")]
pub struct SizesNotEqualError<Index> {
    size0:Index,
    size1:Index
}

impl<Index:PartialEq> SizesNotEqualError<Index> {
    pub fn try_new(size0:Index,size1:Index) -> Result<(),Self> {
        if size0 == size1 {
            Ok(())
        } else {
            Err(Self{size0,size1})
        }
    }

    pub fn try_new_ref(size0:&Index,size1:&Index) -> Result<(),Self> where Index:Clone {
        if size0 == size1 {
            Ok(())
        } else {
            Err(Self{size0:size0.clone(),
                     size1:size1.clone()})
        }
    }

    // panics if it is not error
    pub fn new(size0:Index, size1:Index) -> Self {
        Self::try_new(size0, size1)
            .err()
            .unwrap()
    }
}

impl From<LensNotEqualError> for SizesNotEqualError<usize> {
    fn from(value: LensNotEqualError) -> Self {
        Self{size0:value.len0,size1:value.len1}
    }
}

impl From<SizesNotEqualError<usize>> for LensNotEqualError {
    fn from(value: SizesNotEqualError<usize>) -> Self {
        Self{len0:value.size0,len1:value.size1}
    }
}