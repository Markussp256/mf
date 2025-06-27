use crate::{AnyFromParameters,IntoParameters};

pub trait AnyParameters<F,E> : IntoParameters<F>
                              +AnyFromParameters<F,E> {}

impl<F,E,
     T : IntoParameters<F>
        +AnyFromParameters<F,E>> AnyParameters<F,E> for T {}

pub fn check_x<F,E,X:Clone+AnyParameters<F,E>+Clone+PartialEq>(x:X) {
    if x != X::any_from_iter(Some(&x),x.clone().into_parameters()).ok().unwrap() {
        panic!("IntoParameters is not inverse of AnyFromParameters");
    }
}

pub fn check_f<F:PartialEq,E,X:AnyParameters<F,E>>(iter:impl ExactSizeIterator<Item=F>+Clone) {
    let iter2=X::any_from_iter(None,iter.clone()).ok().unwrap().into_parameters();
    if iter.eq(iter2) {
        panic!("IntoParameter is not inverse of FromParameter");
    }
}