use crate::ContainerIndex;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("lower bound ({lb:?}) is not smaller equal to upper bound ({ub:?})")]
pub struct LowerBoundUpperBoundError<Index> {
    lb:Index,
    ub:Index
}

// we provide reference instead of owned types because we assume that error is rare
// and we only want to copy if it is really necessary

impl<Index:ContainerIndex> LowerBoundUpperBoundError<Index> {
    pub fn try_new(lb:&Index,ub:&Index) -> Result<(),Self> {
        if lb.is_elem_wise_smaller_eq(ub) {
            Ok(())
        } else {
            Err(Self{lb:lb.clone(),ub:ub.clone()})
        }
    }

    // panics if it is not error
    pub fn new(lb:&Index,ub:&Index) -> Self {
        Self::try_new(lb, ub)
            .err()
            .unwrap()
    }
}

impl<Index> LowerBoundUpperBoundError<Index> {

    pub fn into_parts(self) -> (Index,Index) {
        (self.lb, self.ub)
    }
}