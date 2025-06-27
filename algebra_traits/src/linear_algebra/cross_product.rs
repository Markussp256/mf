use std::ops::{Sub, Mul};

pub trait Crossproduct<Rhs=Self> {
    type Output;
    fn cross_product(self, rhs:Rhs) -> Self::Output;
}

impl<T: Clone+Mul<T2, Output=T3>,
     T2:Clone,
     T3:Sub<Output=TR>,
     TR> Crossproduct<[T2;3]> for [T;3] {
        type Output=[TR;3];
        fn cross_product(self, rhs:[T2;3]) -> Self::Output {
            std::array::from_fn(|i|{
                let i1=(i+1) % 3;
                let i2=(i+2) % 3;
                self[i1].clone()*rhs[i2].clone()
               -self[i2].clone()*rhs[i1].clone()})
        }
     }