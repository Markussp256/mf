use super::{TryFromLocalParameters,IntoLocalParameters};

pub trait LocalParameters<F,E> : IntoLocalParameters<F>
                                +TryFromLocalParameters<F,E> {}

impl<F,E,
     T : IntoLocalParameters<F>
        +TryFromLocalParameters<F,E>> LocalParameters<F,E> for T {}

pub fn check_xs<F,E,X:Clone+LocalParameters<F,E>+Clone+PartialEq>(x0:X,x1:X) {
    if x1 != x0.clone().try_from_iter(x0.into_local_parameters(x1.clone())).ok().unwrap() {
        panic!("IntoLocalParameters is not inverse of TryFromLocalParameters");
    }
}

pub fn check_f<F:PartialEq,E,X:Clone+LocalParameters<F,E>>(x:X,iter:impl ExactSizeIterator<Item=F>+Clone) {
    let iter2=x.clone().into_local_parameters(x.try_from_iter(iter.clone()).ok().unwrap());
    if iter.eq(iter2) {
        panic!("IntoLocalParameters is not inverse of TryFromLocalParameters");
    }
}