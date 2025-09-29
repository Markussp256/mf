use nalgebra::{DMatrix, DMatrixView, DMatrixViewMut, Scalar};
use crate::{for_dynamic::{Empty, OneElement}, IsEmpty, OCTSize};

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
}

macro_rules! impl_is_empty {
    ($name:ident $(, $lt:lifetime)?) => {
        impl<$($lt,)? T:Scalar> IsEmpty for $name<$($lt,)?T> {
            fn is_empty(&self) -> bool {
                self.nrows() == 0 && self.ncols() == 0
            }
        }        
    };
}

impl_is_empty!(DMatrix);
impl_is_empty!(DMatrixView,'a);
impl_is_empty!(DMatrixViewMut,'a);