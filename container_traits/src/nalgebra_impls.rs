use nalgebra::Dim;

use crate::{LenNotEqualToRequiredLenError, OtherDimensionMismatchError};

type U2=(usize,usize);

pub trait DimExtension : Dim {
    // panics if Dimension is wrong
    fn new(odyn_len:Option<usize>) -> Self {
        Self::from_usize(odyn_len.or(Self::try_to_usize()).unwrap())
    }

    fn try_new(odyn_len:Option<usize>) -> Result<Self,LenNotEqualToRequiredLenError> {
        if let Some(req_size)=Self::try_to_usize() {
            if let Some(dyn_len) = odyn_len {
                LenNotEqualToRequiredLenError::try_new(req_size, dyn_len)?;
            }
        } else {
            if odyn_len.is_none() {
                panic!("missing dimension");
            }
        }
        Ok(Self::new(odyn_len))
    }
}
impl<D:Dim> DimExtension for D {}


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
mod for_svector;
mod for_smatrix;