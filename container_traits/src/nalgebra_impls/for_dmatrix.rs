use nalgebra::{DMatrix, Scalar};
use crate::for_dynamic::{Empty, OneElement};
type U2=(usize,usize);
use crate::FromFn;

impl<T:Scalar> OneElement<T> for DMatrix<T> {
    fn one_element(t:T) -> Self {
        Self::from_vec(1,1,vec![t])
    }
}

impl<T:Scalar> Empty for DMatrix<T> {
    fn empty() -> Self {
        Self::from_vec(0,0,Vec::new())
    }
}

impl<T:Scalar> FromFn<U2,T> for DMatrix<T> {
    fn from_fn((r,c):U2, f:impl Fn(U2) -> T) -> Self {
        Self::from_fn(r,c, |i,j|f((i,j)))
    }
}