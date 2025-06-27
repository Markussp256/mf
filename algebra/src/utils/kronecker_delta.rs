use num_traits::{Zero, One};
use std::cmp::PartialEq;


pub fn kronecker_delta<I:PartialEq, T:Zero+One>(i:I, j:I) -> T {
    if i == j {
        T::one()
    } else {
        T::zero()
    }
}