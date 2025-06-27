use algebra::VectorDyn;
use container_traits::{IntoInner,FromInner};
use matrix::row_col::MatrixColGeneric;

use super::{Point, UnitVector};

// crate::impl_vector!(Vector, Point);


// #[cfg(test)]
// use algebra_traits::{TryDiv,Scalar, FiniteDimensionalInnerProductSpace, InnerProductSpace1d, Max, Pow2, ScalarMul};

// #[cfg(test)]
// fn test_fin_dim_inner_prod_space<
//     V:TryDiv<Output=F>+InnerProductSpace1d<NormT=NT,Norm2T = NT2>,
//     NT:num_traits::Zero+Max+Pow2<Output=NT2>+ScalarMul<F::RealType>,
//     NT2:num_traits::Zero+Max,
//     F:Scalar>(v:Vector3<V>) -> impl FiniteDimensionalInnerProductSpace<F,3> {
//     v
// }


impl<T, const N:usize> TryFrom<VectorDyn<T>> for Vector<T, N> {
    type Error = VectorDyn<T>;
    fn try_from(value: VectorDyn<T>) -> Result<Self, Self::Error> {
        let mut iter=value.into_iter();
        let oarr:Result<[T;N],Vec<T>>=utils::iter::next_chunk(& mut iter);
        oarr.map(|a|a.into())
            .map_err(|v|v.into())
    }
}


impl<T:Clone, const N:usize> From<UnitVector<T,N>> for Vector<T,N> {
    fn from(value:UnitVector<T,N>) -> Self {
        value.into_inner()
    }
}

#[test]
fn test_vector_zero() {
    use num_traits::Zero;
    let z = Vector::<f64, 3>::zero();
    assert_eq!(&0.0, z.x());
    assert_eq!(&0.0, z.y());
    assert_eq!(&0.0, z.z());
}

// #[macro_export]
// macro_rules! impl_vector {
//      (Vector:ident, Point:ident) => {

algebra::gen_vector!(VectorGen, VectorDynNotUsed, Vector);

macro_rules! from_into {
    ($t:ty) => {
        impl<C:'static> From<$t> for VectorGen<C> {
            fn from(value: $t) -> Self {
                Self(value.into_inner())
            }
        }

        impl<C:'static> Into<$t> for VectorGen<C> {
            fn into(self) -> $t {
                <$t>::from_inner(self.0)
            }
        }
    };
}
from_into!(algebra::VectorGeneric<C>);
from_into!(MatrixColGeneric<C>);



        // #[derive(
        //     Clone,
        //     PartialEq,
        //     Debug,
        //     derive_more::Index,
        //     derive_more::IndexMut,
        //     algebra_derive::Vector,
        //     container_derive::FromFn,
        //     container_derive::IntoIterator,
        //     container_derive::IntoParameters,
        //     container_derive::TryFromParameters,
        //     container_derive::Iter,
        //     container_derive::Map,
        // )]
        // pub struct Vector<T, const N: usize>(EnhancedArray<T,N>);

        // impl<F:Clone+algebra_traits::Scalar, T:algebra_traits::Parameters1<F>, const N:usize> algebra_traits::NumberOfDegreesOfFreedom<F> for Vector<T,N> {
        //     const NDOFS:usize=N;
        // }

        // impl<F:Clone+algebra_traits::Scalar, T:algebra_traits::Parameters1<F>, const N:usize> algebra_traits::Parameters<F> for Vector<T,N> {
        //     fn parameters(&self) -> Vec<F> {
        //         self.iter()
        //             .map(|t|t.parameter())
        //             .collect()
        //     }

        //     fn try_from_iter<I:Iterator<Item=F>>(iter: & mut I) -> Option<Self> {
        //         utils::iter::next_chunk(iter)
        //             .ok()
        //             .map(|arr| Self(arr.map(T::from_parameter)))
        //     }
        // }

        // container_traits_static_impl!(Vector);
        // algebra::scalar_mul_generic!(Vector<T, N>);

        // $crate::geometrie::point::impl_point_or_vector!(Vector); //
        #[cfg(feature = "nalgebra_support")]
        impl<T: nalgebra::Scalar, const N: usize> From<nalgebra::SVector<T, N>>
            for Vector<T, N>
        {
            fn from(v: nalgebra::SVector<T, N>) -> Self {
                Self::from_fn(|i| v[i].clone())
            }
        }

        #[cfg(feature = "nalgebra_support")]
        impl<T: nalgebra::Scalar, const N: usize> Into<nalgebra::SVector<T, N>>
            for Vector<T, N>
        {
            fn into(self) -> nalgebra::SVector<T, N> {
                nalgebra::SVector::from_fn(|i, _| self[i].clone())
            }
        }

        #[cfg(feature = "cgmath_support")]
        impl From<cgmath::Vector2<f64>> for Vector<f64,2> {
            fn from(p: cgmath::Vector2<f64>) -> Self {
                Self::new(p.x,p.y)
            }
        }

        #[cfg(feature = "cgmath_support")]
        impl From<cgmath::Vector3<f64>> for Vector<f64,3> {
            fn from(p: cgmath::Vector3<f64>) -> Self {
                Self::new(p.x,p.y,p.z)
            }
        }

        #[cfg(feature = "cgmath_support")]
        impl Into<cgmath::Vector2<f64>> for Vector<f64,2> {
            fn into(self) -> cgmath::Vector2<f64> {
                let [x,y]=self.0;
                cgmath::Vector2::new(x,y)
            }
        }

        #[cfg(feature = "cgmath_support")]
        impl Into<cgmath::Vector3<f64>> for Vector<f64,3> {
            fn into(self) -> cgmath::Vector3<f64> {
                let [x,y,z]=self.0;
                cgmath::Vector3::new(x,y,z)
            }
        }

        // vector operations

        // impl<V:Clone+std::ops::Sub<Output=V>> Vector<V,3> {
        //     pub fn cross_product<F:Clone, VF:Sub<Output=VF>>(&self, rhs: & Vector<F, 3>) -> Vector<VF, 3> where V:std::ops::Mul<F, Output=VF> {
        //         let sa:[V;3]=self.clone().into();
        //         let rhsa:[F;3]=rhs.clone().into();
        //         let res_arr:[V;3]=utils::array_op::cross_product::<V,F,VF,VF>(sa, rhsa);
        //         Vector::from(res_arr)
        //     }
        // }


        // end vector operations

        // impl<V : algebra_traits::Norm<NormT=NormT>,
        //      NormT: algebra_traits::Max+num_traits::Zero, const N:usize> Vector<V, N> {
        //     pub fn max_norm_of_entries(self) -> algebra_traits::Nonnegative<NormT> {
        //         algebra::utils::max_norm(self.0)
        //     }
        // }

        // norm for e.g. physical quantities
        // provided with trait scalar_product
        // impl<T: Clone+algebra_traits::ConstNonzero + std::ops::Div<Output=f64>, const N:usize> Vector<T,N> {
        //     pub fn norm(&self) -> T {
        //         T::NonZero * <Vector::<f64,N> as algebra_traits::Scalarproduct<f64>>::norm(&self.clone().map(|z|z/T::NonZero))
        //     }
        // }

        // vector-point operations
        impl<T: std::ops::Sub<T2, Output = T3>, T2, T3, const N: usize>
            std::ops::Sub<Point<T2, N>> for Point<T, N>
        {
            type Output = Vector<T3, N>;
            fn sub(self, rhs: Point<T2, N>) -> Self::Output {
                (self.into_inner()-rhs.into_inner()).into()
            }
        }

        impl<T: std::ops::Sub<T2, Output = T3>, T2, T3, const N: usize>
            std::ops::Sub<Vector<T2, N>> for Point<T, N>
        {
            type Output = Point<T3, N>;
            fn sub(self, rhs: Vector<T2, N>) -> Self::Output {
                (self.into_inner()-rhs.into_inner()).into()
            }
        }

        impl<T: std::ops::Add<T2, Output = T3>, T2, T3, const N: usize>
            std::ops::Add<Vector<T2, N>> for Point<T, N>
        {
            type Output = Point<T3, N>;
            fn add(self, rhs: Vector<T2, N>) -> Self::Output {
                (self.into_inner()+rhs.into_inner()).into()
            }
        }

        impl<A:algebra_traits::Torsor, const N:usize> algebra_traits::Torsor for Point<A, N> {}

        impl<A: std::ops::Sub<Output=V>, V, const N:usize> algebra_traits::Distance for Point<A, N>
        where Vector<V,N> : algebra_traits::Norm {
            type DistT=<Vector<V,N> as algebra_traits::Norm>::NormT;
            fn distance(self, rhs:impl Into<Self>) -> algebra_traits::Nonnegative<Self::DistT> {
                let rhs:Self=rhs.into();
                <Vector::<V,N> as algebra_traits::Norm>::norm(rhs-self)
            }
        }
        
        impl<A:std::ops::Sub<Output=V>+algebra_traits::MetricTorsor, V, const N:usize> algebra_traits::MetricTorsor for Point<A, N>
        where Vector<V,N> : algebra_traits::Norm {}


        impl<A: std::ops::Sub<Output=V>+algebra_traits::Tolerance<DistT=NormT>,
             V,
             NormT,
             const N:usize> algebra_traits::Tolerance for Point<A, N>
        where Vector<V,N> : algebra_traits::Norm<NormT=NormT>,
        NormT : std::cmp::PartialOrd+num_traits::Zero+algebra_traits::Max { 
            const THRESHOLD:NormT=<A as algebra_traits::Tolerance>::THRESHOLD;
        }
        

        // utils::from_via!(impl<T, const N:usize> From<MatrixCol<T,N>> for Vector<T,N>, via [T;N]);

        // impl<T, const N:usize> Into<algebra::Vector<T, N>> for Vector<T, N> {
        //     fn into(self) -> algebra::Vector<T, N> {
        //         self.0.into()
        //     }
        // }

        utils::from_via!(impl<T, const N:usize> From<algebra::Vector<T, N>> for Vector<T, N>, via [T;N]);

        impl<V:algebra_traits::Tolerance, const N:usize> algebra_traits::Tolerance for Vector<V, N>
        where Self : algebra_traits::Distance<DistT=<V as algebra_traits::Distance>::DistT>,
              Self::DistT:PartialOrd {
            const THRESHOLD:<V as algebra_traits::Distance>::DistT=<V as algebra_traits::Tolerance>::THRESHOLD;
        }


//     }
// }






// impl<const N:usize> algebra_traits::Scalarproduct<f64> for Vector<phys_units::Length, N> {
//     type N=phys_units::Length;
//     type N2=phys_units::Area;

//     fn scalar_product(self, rhs: Self) -> Self::N2 {
//         <phys_units::Area as algebra_traits::AdditiveGroup>::sum(self.into_iter().zip(rhs.into_iter()).map(|(a,b)|a*b))
//     }

//     fn sqrt(n2:Self::N2) -> Self::N {
//         phys_units::Area::sqrt(n2)
//     }
// }


#[test]
fn test_scalar_product_with_lengths() {
    use algebra_traits::{Sqrt, Nonnegative, NormSquared};
    use container_traits::Map;
    use phys_units::{Meters, Length, Area};

    let v=Vector::<f64,3>::new(3.0,0.0,4.0).map(Length::from_m);
    
    let n2=<Vector3<Length> as NormSquared>::norm_squared(v.clone());
    let n=<Nonnegative<Area> as Sqrt>::sqrt(n2);
    assert_eq!(Length::from_m(5.0), n);
}