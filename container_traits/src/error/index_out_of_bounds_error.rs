use crate::ContainerIndex;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("index ({index}) is out of bounds ({bounds})")]
pub struct IndexOutOfBoundsError<Index> {
    index:Index,
    bounds:Index
}

// we provide reference instead of owned types because we assume that error is rare
// and we only want to copy if it is really necessary

impl<Index:ContainerIndex> IndexOutOfBoundsError<Index> {
    pub fn try_new(size:&Index,index:&Index) -> Result<(),Self> {
        if index.is_elem_wise_strictly_smaller(size) {
            Ok(())
        } else {
            Err(Self{index:index.clone(),bounds:size.clone()})
        }
    }

    // panics if it is not error
    pub fn new(size:&Index,index:&Index) -> Self {
        Self::try_new(size, index)
            .err()
            .unwrap()
    }
}
