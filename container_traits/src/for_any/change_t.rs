pub trait ChangeT<T2> {
    type Output;
}

impl<T,T2> ChangeT<T2> for Vec<T> {
    type Output = Vec<T2>;
}

impl<T,T2, const N:usize> ChangeT<T2> for [T;N] {
    type Output = [T2;N];
}

#[cfg(feature = "nalgebra_support")]
mod impl_nalgebra {
    use super::ChangeT;
    use nalgebra::{Scalar,Dim};
    use nalgebra::base::{ArrayStorage, VecStorage, ViewStorageMut, ViewStorage};

    impl<T:Scalar, T2:Scalar, const R:usize, const C:usize> ChangeT<T2> for ArrayStorage<T,R,C> {
        type Output=ArrayStorage<T2,R,C>;
    }

    impl<T:Scalar, T2:Scalar, R:Dim, C:Dim> ChangeT<T2> for VecStorage<T,R,C> {
        type Output=VecStorage<T2,R,C>;
    }

    impl<'a,T:Scalar, T2:Scalar, R:Dim, C:Dim, RStride:Dim, CStride:Dim> ChangeT<T2> for ViewStorageMut<'a,T,R,C,RStride,CStride> {
        type Output=ViewStorageMut<'a,T2,R,C,RStride,CStride>;
    }

    impl<'a,T:Scalar, T2:Scalar, R:Dim, C:Dim, RStride:Dim, CStride:Dim> ChangeT<T2> for ViewStorage<'a,T,R,C,RStride,CStride> {
        type Output=ViewStorage<'a,T2,R,C,RStride,CStride>;
    }
}