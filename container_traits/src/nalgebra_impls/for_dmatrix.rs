use nalgebra::{DMatrix, Scalar};
use crate::for_dynamic::{Empty, OneElement};

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