
#[macro_export]
macro_rules! gen_vector_view {
    ($gen:ident, $dyn:ident, $stat:ident) => {
        #[derive(Clone, Debug, PartialEq,
            container_derive::ContainerView,
            algebra_derive::Norm,
            derive_more::AsRef,
            derive_more::From,
            derive_more::Index,
        )]
        pub struct $gen<C>(C);

        pub type $dyn<'a,T>=$gen<$crate::EnhancedVec<&'a T>>;
        pub type $stat<'a,T,N:generic_array::ArrayLength>=$gen<$crate::EnhancedArray<&'a T,N>>;

        impl<'a,
             C : container_traits::Iter<T>
                +container_traits::ItemT<T=&'a T>,
             T :'static> $gen<C> {
            pub fn try_max(&'a self) -> Option<&'a T> where T : 'static+algebra_traits::Max {
                self.0
                    .iter()
                    .reduce(|a,b| if a > b { a } else { b })
            }
        }

        impl<'a,C:container_traits::LinearContainerView<T=T>, T:std::fmt::Display> std::fmt::Display for $gen<C> {
            container_traits::impl_display_for_linear_container!();
        }

        impl<'a, C : algebra_traits::Norm<NormT=NT>,
             NT: algebra_traits::RealNumber> TryFrom<$gen<C>> for $crate::Unit<$gen<C>> {
            type Error=$gen<C>;
            fn try_from(value:$gen<C>) -> Result<Self, $gen<C>> {
                Self::try_new(value)
            }
        }

        impl<'a,T> Into<Vec<&'a T>> for $dyn<'a,T> {
            fn into(self) -> Vec<&'a T> {
                self.0
                    .into()
            }
        }

        impl<'a,T> From<Vec<&'a T>> for $dyn<'a,T> {
            fn from(v:Vec<&'a T>) -> Self {
                $crate::EnhancedVec::<&'a T>::from(v).into()
            }
        }

        impl<'a,T,const N:usize> From<[&'a T;N]> for $dyn<'a,T> {
            fn from(arr:[&'a T;N]) -> Self {
                Vec::<&'a T>::from(arr).into()
            }
        }

        impl<'a,T,N:generic_array::ArrayLength> From<$stat<'a,T,N>> for $dyn<'a,T> {
            fn from(arr:$stat<'a,T,N>) -> Self {
                let v:Vec<&'a T>=<$crate::EnhancedArray<&'a T,N> as container_traits::IntoIter<&'a T>>::into_iterator(arr.0).collect();
                v.into()
            }
        }       

        impl<'a,T,const N:usize> TryInto<[&'a T;N]> for $dyn<'a,T> {
            type Error=Self;
            fn try_into(self) -> Result<[&'a T;N],Self> {
                let v:Vec<&'a T>=self.into();
                v.try_into()
                 .map_err(|v:Vec<&'a T>|v.into())
            }
        }

        impl<'a,T> $stat<'a,T,generic_array::typenum::U2> {
            pub const fn new(x:&'a T,y:&'a T) -> Self {
                Self($crate::EnhancedArray::from_array([x,y]))
            }

            pub fn x(&'a self) -> &'a T { &self[0] }
            pub fn y(&'a self) -> &'a T { &self[1] }
        }

        impl<'a,T> $stat<'a,T,generic_array::typenum::U3> {
            pub const fn new(x:&'a T,y:&'a T,z:&'a T) -> Self {
                Self($crate::EnhancedArray::from_array([x,y,z]))
            }

            pub fn x(&'a self) -> &'a T { &self[0] }
            pub fn y(&'a self) -> &'a T { &self[1] }
            pub fn z(&'a self) -> &'a T { &self[2] }
        }

        paste::paste!(
            pub type [<$stat 2>]<'a,T>=$stat<'a,T,generic_array::typenum::U2>;
            pub type [<$stat 3>]<'a,T>=$stat<'a,T,generic_array::typenum::U3>;
            pub type [<$stat 4>]<'a,T>=$stat<'a,T,generic_array::typenum::U4>;
        );

        impl<'a, T,N:generic_array::ArrayLength> Into<$crate::EnhancedArray<&'a T,N>> for $stat<'a,T,N> {
            fn into(self) -> $crate::EnhancedArray<&'a T,N> {
                self.0
            }
        }

        impl<'a, T, N:generic_array::ArrayLength> $stat<'a,T,N> {
            fn into_array<const U:usize>(self) -> [&'a T;U]
            where typenum::Const<U>: generic_array::IntoArrayLength<ArrayLength = N> {
                self.0
                    .into_array::<U>()
            }

            fn from_array<const U:usize>(arr:[&'a T;U]) -> Self
            where typenum::Const<U>: generic_array::IntoArrayLength<ArrayLength = N> {
                $crate::EnhancedArray::<&'a T,N>::from_array(arr).into()
            }
        }
        
        impl<'a, T, N:generic_array::ArrayLength> Into<Vec<&'a T>> for $stat<'a, T,N> {
            fn into(self) -> Vec<&'a T> {
                <$crate::EnhancedArray<&'a T,N> as container_traits::IntoInner>::into_inner(self.0)
                    .into_iter()
                    .collect()
            }
        }

        impl<'a, T, N:generic_array::ArrayLength> TryFrom<Vec<&'a T>> for $stat<'a,T,N> {
            type Error=container_traits::LenNotEqualToRequiredLenError;
            fn try_from(v:Vec<&'a T>) -> Result<Self,Self::Error> {
                <$crate::EnhancedArray<&'a T, N> as TryFrom<Vec<&'a T>>>::try_from(v)
                    .map(|a|Self(a))
            }
        }

        impl<'a, T, N:generic_array::ArrayLength> TryFrom<$dyn<'a,T>> for $stat<'a,T,N> {
            type Error=container_traits::LenNotEqualToRequiredLenError;
            fn try_from(v:$dyn<'a,T>) -> Result<Self,Self::Error> {
                let v:Vec<&'a T>=<$crate::EnhancedVec<&'a T> as container_traits::IntoInner>::into_inner(v.0);
                <$crate::EnhancedArray<&'a T, N> as TryFrom<Vec<&'a T>>>::try_from(v)
                    .map(|a|Self(a))
            }
        }

        impl<'a, T> Into<$crate::EnhancedVec<&'a T>> for $dyn<'a,T> {
            fn into(self) -> $crate::EnhancedVec<&'a T> {
                self.0
            }
        }

            // pub fn from_unchecked_view<C2:'static+Into<C>>(value:$gen<C2>) -> Self {
            //     Self(value.0.into())
            // }

            // pub fn into_unchecked_view<C2:'static>(self) -> $gen<C2> where C:Into<C2> {
            //     $gen::from_unchecked_view(self)
            // }
            // pub fn try_from_unchecked_view<C2:'static>(value:$gen<C2>) -> Result<Self, $gen<C2>> where C2 : TryInto<C,Error=C2> {
            //     value.0
            //          .try_into()
            //          .map(|c|$gen(c))
            //          .map_err(|c2|$gen(c2))
            // }
            // pub fn try_into_unchecked_view<C2:'static>(self) -> Result<$gen<C2>, Self> where C:TryInto<C2,Error=C> {
            //     $gen::try_from_unchecked_view(self)
            // }

    };
}

gen_vector_view!(VectorViewGeneric, VectorViewDyn, VectorView);