use num_traits::{Zero,One};


pub fn kron_delta<I:PartialEq,F:Zero+One>(i0:I,i1:I) -> F {
    if i0 == i1 {
        F::one()
    } else {
        F::zero()
    }
}