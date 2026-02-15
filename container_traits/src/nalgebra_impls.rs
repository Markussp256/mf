use nalgebra::{Const, Dim, Dyn};

use crate::{LenNotEqualToRequiredLenError, OtherDimensionMismatchError};

type U2=(usize,usize);

pub trait DimExtension : Dim {

    const VALUE:Option<usize>;
    // can panic
    fn new(odyn_len:Option<usize>) -> Self;

    fn try_new(odyn_len:Option<usize>) -> Result<Self,LenNotEqualToRequiredLenError>;
}
impl DimExtension for Dyn {
    const VALUE:Option<usize> = None;
    // panics if Dimension is missing
    fn new(odyn_len:Option<usize>) -> Self {
        Self::from_usize(odyn_len.unwrap())
    }
    fn try_new(odyn_len:Option<usize>) -> Result<Self,LenNotEqualToRequiredLenError> {
        if odyn_len.is_none() {
            panic!("missing dimension");
        }
        Ok(<Self as DimExtension>::new(odyn_len))
    }
}

impl<const N:usize> DimExtension for Const<N> {
    const VALUE:Option<usize> = Some(N);
    // panics if Dimension is wrong
    fn new(odyn_len:Option<usize>) -> Self {
        Self::from_usize(odyn_len.or(Self::VALUE).unwrap())
    }

    fn try_new(odyn_len:Option<usize>) -> Result<Self,LenNotEqualToRequiredLenError> {
        if let Some(req_size)=Self::VALUE {
            if let Some(dyn_len) = odyn_len {
                LenNotEqualToRequiredLenError::try_new(req_size, dyn_len)?;
            }
        }
        Ok(Self::new(odyn_len))
    }
}

pub fn get_dims_from_len<R:Dim,C:Dim>(len:usize) -> Result<U2,OtherDimensionMismatchError> {
    match (R::try_to_usize(),C::try_to_usize()) {
        (Some(nrows), Some(ncols))  =>  (nrows*ncols == len).then_some((nrows,     ncols)),
        (Some(nrows), None)                =>  (len % nrows == 0  ).then_some((nrows,     len/nrows)),
        (None,               Some(ncols))  =>  (len % ncols == 0  ).then_some((len/ncols, ncols)),
        (None,               None)                =>  Some((len,1))
    }.ok_or(OtherDimensionMismatchError.into())
}


// unfortunately we can not implement traits for nalgebra row and colvector separately because then 1x1 vector would be defined twice
// we therefore define traits for general matrix such that its correct for row and colvector

mod for_any;
mod for_dvector;
mod for_dmatrix;
mod for_omatrix;
mod for_svector;
mod for_smatrix;

// use nalgebra::*;
// use crate::OCTSize;

// fn test<F0  : Scalar,
//         D0  : Dim,
//         RS0 : RawStorage<F0,Const<1>,D0>>(v:nalgebra::Matrix<F0,Const<1>,D0,RS0>) -> impl OCTSize<usize> {
//             v
// }