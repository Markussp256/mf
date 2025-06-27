
use crate::VectorDyn;
use container_traits::{FromFn, Len, for_static};


impl<T:Clone> From<nalgebra::DVector<T>> for VectorDyn<T> {
    fn from(value:nalgebra::DVector<T>) -> Self {
        <Self as FromFn<usize,T>>::from_fn(value.len(),|i|value[i].clone())
    }
}

impl<T:Clone, const N:usize> From<nalgebra::SVector<T, N>> for super::Vector<T, N> {
    fn from(value:nalgebra::SVector<T, N>) -> Self {
        <Self as for_static::FromFn<usize,T>>::from_fn(|i|value[i].clone())
    }
}

impl<T:Clone+nalgebra::Scalar> Into<nalgebra::DVector<T>> for VectorDyn<T> {
    fn into(self) -> nalgebra::DVector<T> {
        nalgebra::DVector::<T>::from_fn(self.len(),|i,_|self[i].clone())
    }
}

impl<T:Clone+nalgebra::Scalar, const N:usize> Into<nalgebra::SVector<T,N>> for super::Vector<T, N> {
    fn into(self) -> nalgebra::SVector<T,N> {
        nalgebra::SVector::<T,N>::from_fn(|i,_|self[i].clone())
    }
}

// vectordyn does not satisfy Scalarproduct, vectors could have different lengths
// algebra_traits::impl_norm_squared_from_iter!(VectorDyn<T>);
// algebra_traits::impl_norm_for_vector!(VectorDyn<T>);

// fn test_try_normalize<T:algebra_traits::Scalar>() -> UnitVectorDyn<T> {
//     use algebra_traits::TryNormalize;
//     let v=VectorDyn::<T>::zeros(5);
//     <VectorDyn::<T> as TryNormalize>::try_normalize(&v).unwrap()
// }

// crate::scalar_mul_generic!(VectorDyn<T>);
