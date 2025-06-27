use crate::IntoParameters;
use super::TryFromParameters;

pub trait TryParameters<F,E> : IntoParameters<F>
                              +TryFromParameters<F,E> {}

impl<F,E,
     T : IntoParameters<F>
        +TryFromParameters<F,E>> TryParameters<F,E> for T {}

pub fn check_x<F,E,X:Clone+TryParameters<F,E>+Clone+PartialEq>(x:X) {
    if x != X::try_from_iter(x.clone().into_parameters()).ok().unwrap() {
        panic!("IntoParameters is not inverse of AnyFromParameters");
    }
}

pub fn check_f<F:PartialEq,E,X:TryParameters<F,E>>(iter:impl ExactSizeIterator<Item=F>+Clone) {
    let iter2=X::try_from_iter(iter.clone()).ok().unwrap().into_parameters();
    if iter.eq(iter2) {
        panic!("IntoParameter is not inverse of FromParameter");
    }
}