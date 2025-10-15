
// returns corresponding statically sized type that holds container of size (R,C)
pub trait ChangeDim {
    type Output<const R:usize,const C:usize>;
}

// #[cfg(feature = "nalgebra_support")]
// mod impl_nalgebra {
//     use super::ChangeDim;
//     use nalgebra::{Const, Dim, Scalar};
//     use nalgebra::base::{ArrayStorage, VecStorage, ViewStorageMut, ViewStorage};

//     impl<T:Scalar, const R2:usize, const C2:usize> ChangeDim for ArrayStorage<T,R2,C2> {
//         type Output<const R:usize,const C:usize> = ArrayStorage<T,R,C>;
//     }

//     impl<T:Scalar, R:Dim, C:Dim> ChangeDim for VecStorage<T,R,C> {
//         type Output<const R2:usize,const C2:usize> = ArrayStorage<T,R2,C2>;
//     }

//     impl<'a,T:Scalar, R:Dim, C:Dim, RStride:Dim, CStride:Dim> ChangeDim for ViewStorageMut<'a,T,R,C,RStride,CStride> {
//         type Output<const R2:usize,const C2:usize> = ViewStorageMut<'a,T,Const<R2>,Const<C2>,Const<1>,Const<R2>>;
//     }

//     impl<'a,T:Scalar, R:Dim, C:Dim, RStride:Dim, CStride:Dim> ChangeDim for ViewStorage<'a,T,R,C,RStride,CStride> {
//         type Output<const R2:usize,const C2:usize> = ViewStorage<'a,T,Const<R2>,Const<C2>,Const<1>,Const<R2>>;
//     }
// }