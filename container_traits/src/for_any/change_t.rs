use generic_array::{ArrayLength, GenericArray};

pub trait ChangeT<T2> {
    type Output<'a>;
}

impl<T,T2> ChangeT<T2> for Vec<T> {
    type Output<'a> = Vec<T2>;
}

impl<T,T2, N : ArrayLength> ChangeT<T2> for GenericArray<T,N> {
    type Output<'a> = GenericArray<T2,N>;
}

impl<T,T2, const N:usize> ChangeT<T2> for [T;N] {
    type Output<'a> = [T2;N];
}

#[cfg(feature = "nalgebra_support")]
mod impl_nalgebra {
    use super::ChangeT;
    use nalgebra::{Scalar,Dim};
    use nalgebra::base::{ArrayStorage, VecStorage, ViewStorageMut, ViewStorage};

    impl<T:Scalar, T2:Scalar, const R:usize, const C:usize> ChangeT<T2> for ArrayStorage<T,R,C> {
        type Output<'a>=ArrayStorage<T2,R,C>;
    }

    impl<T:Scalar, T2:Scalar, R:Dim, C:Dim> ChangeT<T2> for VecStorage<T,R,C> {
        type Output<'a>=VecStorage<T2,R,C>;
    }

    impl<'v,T:Scalar,T2:Scalar,R:Dim,C:Dim,RStride:Dim,CStride:Dim> ChangeT<T2> for ViewStorage<'v,T,R,C,RStride,CStride>
    {
        type Output<'a> = ViewStorage<'a,T2,R,C,RStride,CStride>;
    }

    impl<'v,T:Scalar,T2:Scalar,R:Dim,C:Dim,RStride:Dim,CStride:Dim> ChangeT<T2> for ViewStorageMut<'v,T,R,C,RStride,CStride>
    {
        type Output<'a> = ViewStorageMut<'a,T2,R,C,RStride,CStride>;
    }
}