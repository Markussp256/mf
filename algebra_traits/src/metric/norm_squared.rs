use crate::{Max, Nonnegative};

use std::ops::Add;
use num_traits::Zero;

pub trait NormSquared {
    type Norm2T; //:Zero+Max
    // required method
    fn norm_squared(self) -> Nonnegative<Self::Norm2T>;
}


// #[cfg(feature = "nalgebra_support")]
// impl<T:nalgebra::Scalar+std::ops::Mul<Output=T2>, T2:num_traits::Zero+PartialOrd, const N:usize> NormSquared for nalgebra::SVector<T,N> {
//     type Norm2T=T2;

//     fn norm_squared(self) -> Nonnegative<Self::Norm2T> {
//         self.iter()
//             .map(|vi|Nonnegative::try_new(vi.clone()*vi.clone()).unwrap())
//             .fold(Nonnegative::zero(),|acc,new|acc+new)
//     }
// }

fn norm_squared_impl<T:NormSquared<Norm2T = TR>, TR:Zero+Max>(s:impl IntoIterator<Item=T>) -> Nonnegative<TR> {
    s.into_iter()
     .map(NormSquared::norm_squared)
     .fold(Nonnegative::zero(),|acc,new|acc.add(new))
}

impl<T:NormSquared<Norm2T = TR>, TR: Zero+Max> NormSquared for Vec<T> {
    type Norm2T=TR;
    fn norm_squared(self) -> Nonnegative<Self::Norm2T> {
        norm_squared_impl(self)
    }
}

impl<T:NormSquared<Norm2T = TR>, TR: Zero+Max, const N:usize> NormSquared for [T;N] {
    type Norm2T=TR;
    fn norm_squared(self) -> Nonnegative<Self::Norm2T> {
        norm_squared_impl(self)
    }
}


#[test]
fn test_norm_squared() {
    let v=vec![0.8,0.6];
    assert_eq!(v.norm_squared(), 1.0)
}

// #[derive(Clone)]
// struct Foo;


// impl<T:Norm<P=f64,S=f64>> Norm for Vec<T> {

// }
