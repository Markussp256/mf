use num_traits::{Zero,One};
use container_traits::ContainerDynamicallySized;
use crate::MatrixConstructError;

use super::MatrixConstruct;
use utils::kron_delta;

type U2=(usize,usize);

pub trait MatrixDynamicallySized : MatrixConstruct + ContainerDynamicallySized<U2,MatrixConstructError> {

    // provided
    fn scalar(t:Self::T) -> Self {
        Self::one_element(t)
    }

    fn identity(n:usize) -> Self where Self::T:Zero+One {
        Self::from_fn((n,n),|(i,j)|kron_delta(i, j))
    }
}