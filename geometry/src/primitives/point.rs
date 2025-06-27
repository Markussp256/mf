crate::impl_point!(Point);

macro_rules! impl_point_or_vector {
        ($pvname:ident $(, Vectorspace<$field:ident>)?) => {

        paste::paste!(
            pub type [<$pvname 2>]<T> = $pvname<T,2>;
            impl<T$(:Vectorspace<$field>)?> [<$pvname 2>]<T>  {
                pub const fn new(x:T,y:T) -> Self {
                    Self(algebra::EnhancedArray::new([x,y]))
                }
            }

            impl<T$(:Vectorspace<$field>)?> [<$pvname 2>]<T>  {
                pub fn x(&self) -> &T {
                    &self.0[0]
                }
                pub fn y(&self) -> &T {
                    &self.0[1]
                }
            }

            pub type [<$pvname 3>]<T> = $pvname<T,3>;
            impl<T$(:Vectorspace<$field>)?> [<$pvname 3>]<T>  {
                pub const fn new(x:T,y:T,z:T) -> Self {
                    Self(algebra::EnhancedArray::new([x,y,z]))
                }
            }

            impl<T$(:Vectorspace<$field>)?> [<$pvname 3>]<T>  {
                pub fn x(&self) -> &T {
                    &self.0[0]
                }
                pub fn y(&self) -> &T {
                    &self.0[1]
                }
                pub fn z(&self) -> &T {
                    &self.0[2]
                }
            }
        );

        // impl<T$(:Vectorspace<$field>)?,const N:usize> $pvname<T,N>  {

        //     pub fn from_fn<F>(f:F) -> Self where F:Fn(usize)-> T {
        //         Self(core::array::from_fn(f))
        //     }

        //     pub fn iter(&self) -> std::slice::Iter<T> {
        //         self.0.iter()
        //     }

        //     pub fn try_max(self) -> Option<T> where T:algebra_traits::Max {
        //         crate::algebra::array::try_max(self.0)
        //     }

        //     pub fn try_max_norm(&self) -> Option<T> where T: algebra_traits::Max
        //                                                     +algebra_traits::Signed+Clone+PartialOrd {
        //         crate::algebra::array::try_max_norm(self.0.clone())
        //     }

        //     pub fn iter_mut(&mut self) -> std::slice::IterMut<'_,T> {
        //         self.0.iter_mut()
        //     }

        //     pub fn map<T2$(:Vectorspace<$field>)?>(self, f: impl Fn(T) -> T2) -> $pvname<T2,N> {
        //         $pvname::<T2,N>(self.0.map(f))
        //     }
        // }

        // impl<T$(:Vectorspace<$field>)?,const N:usize> From<[T;N]> for $pvname<T,N> {
        //     fn from(value:[T;N]) -> Self {
        //         Self(value)
        //     }
        // }

        // impl<T$(:Vectorspace<$field>)?,const N:usize> Into<[T;N]> for $pvname<T,N> {
        //     fn into(self) -> [T;N] {
        //         self.0
        //     }
        // }

        // impl<T:Copy$(+Vectorspace<$field>)?,const N:usize> $pvname<T,N>  {
        //     pub fn from_element(t:T) -> Self {
        //         Self([t;N])
        //     }
        // }

        // impl<T:algebra_traits::ConstElement> algebra_traits::ConstElement for $pvname<T,2> {
        //     const ELEMENT:Self=Self::new(T::ELEMENT,T::ELEMENT);
        // }

        // impl<T:algebra_traits::ConstElement> algebra_traits::ConstElement for $pvname<T,3> {
        //     const ELEMENT:Self=Self::new(T::ELEMENT,T::ELEMENT,T::ELEMENT);
        // }
    }
}
pub(crate) use impl_point_or_vector;


#[macro_export]
macro_rules! impl_point {
    ($pname:ident) => {
        #[derive(
            Clone,
            PartialEq,
            Debug,
            algebra_derive::Basis,
            algebra_derive::ConstElement,
            container_derive::Container,
            derive_more::AsRef,
            derive_more::AsMut,
            derive_more::From,
            derive_more::Index,
            derive_more::IndexMut,
        )]
        pub struct $pname<T, const N: usize>(algebra::EnhancedArray<T, N>);

        $crate::point::impl_point_or_vector!($pname);
        // container_traits_static_impl!($pname);


        impl<T: algebra_traits::Origin, const N: usize> algebra_traits::Origin for $pname<T, N> {
            fn origin() -> Self {
                <Self as container_traits::for_static::FromFn<usize,T>>::from_fn(|_|<T as algebra_traits::Origin>::origin())
            }
        }

        // impl<F:algebra_traits::Scalar,
        //      V:algebra_traits::TryDiv<Output=F>+algebra_traits::Vectorspace1d,
        //      A:Clone+std::ops::Sub<Output=V>+algebra_traits::Torsor+algebra_traits::ConstElement,
        //      const N:usize> algebra_traits::Basis<F> for $pname<A, N> where Self : algebra_traits::Origin {
        //         fn basis() -> Vec<Self> {
        //         let v_basis=<$pname<A, N> as std::ops::Sub>::Output::basis();
        //         let orig=||<Self as algebra_traits::Origin>::origin();
        //         std::iter::once(orig())
        //             .chain(v_basis.into_iter().map(|v|orig()+v))
        //             .collect()
        //     }
        // }

        #[cfg(feature = "nalgebra_support")]
        impl<T: nalgebra::Scalar, const N: usize> From<nalgebra::Point<T, N>> for $pname<T, N> {
            fn from(p: nalgebra::Point<T, N>) -> Self {
                Self::from_fn(|i| p[i].clone())
            }
        }

        #[cfg(feature = "nalgebra_support")]
        impl<T: nalgebra::Scalar, const N: usize> Into<nalgebra::Point<T, N>> for $pname<T, N> {
            fn into(self) -> nalgebra::Point<T, N> {
                nalgebra::Point::from(<[T;N]>::from(self))
            }
        }

        #[cfg(feature = "cgmath_support")]
        impl From<cgmath::Point2<f64>> for $pname<f64,2> {
            fn from(p: cgmath::Point2<f64>) -> Self {
                Self::new(p.x, p.y)
            }
        }

        #[cfg(feature = "cgmath_support")]
        impl From<cgmath::Point3<f64>> for $pname<f64,3> {
            fn from(p: cgmath::Point3<f64>) -> Self {
                Self::new(p.x, p.y, p.z)
            }
        }

        #[cfg(feature = "cgmath_support")]
        impl Into<cgmath::Point2<f64>> for $pname<f64,2> {
            fn into(self) -> cgmath::Point2<f64> {
                let [x,y]=self.0;
                cgmath::Point2::new(x,y)
            }
        }

        #[cfg(feature = "cgmath_support")]
        impl Into<cgmath::Point3<f64>> for $pname<f64,3> {
            fn into(self) -> cgmath::Point3<f64> {
                let [x,y,z]=self.0;
                cgmath::Point3::new(x, y, z)
            }
        }


    };
}