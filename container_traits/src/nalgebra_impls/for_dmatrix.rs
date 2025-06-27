use nalgebra::{DMatrix,Scalar};
use crate::{for_dynamic::{Empty, OneElement}, OCTSize};

type U2=(usize,usize);

impl<T:Scalar> OneElement<T> for DMatrix<T> {
    fn one_element(t:T) -> Self {
        Self::from_vec(1,1,vec![t])
    }
}

impl<T:Scalar> OCTSize<U2> for DMatrix<T> {
    const OCTSIZE:Option<U2>=None;
}

impl<T:Scalar> Empty for DMatrix<T> {
    fn empty() -> Self {
        Self::from_vec(0,0,Vec::new())
    }

    fn is_empty(&self) -> bool {
        self.nrows() == 0 && self.ncols() == 0
    }
}