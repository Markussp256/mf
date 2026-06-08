

#[macro_export]
macro_rules! gen_vector_view_mut {
    ($gen:ident, $dyn:ident, $stat:ident) => {
        #[derive(Clone, Debug, PartialEq,
            container_derive::ContainerViewMut,
            container_derive::TryFromIterator,
            algebra_derive::Norm,
            derive_more::AsRef,
            derive_more::AsMut,
            derive_more::From,
            derive_more::Index,
            derive_more::IndexMut,
        )]
        pub struct $gen<C>(C);

        pub type $dyn<'a,T>=$gen<$crate::EnhancedVec<&'a mut T>>;
        pub type $stat<'a,T,N:generic_array::ArrayLength>=$gen<$crate::EnhancedArray<&'a mut T,N>>;

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

        impl<'a,T> Into<Vec<&'a mut T>> for $dyn<'a,T> {
            fn into(self) -> Vec<&'a mut T> {
                self.0
                    .into()
            }
        }

        impl<'a,T> From<Vec<&'a mut T>> for $dyn<'a,T> {
            fn from(v:Vec<&'a mut T>) -> Self {
                $crate::EnhancedVec::<&'a mut T>::from(v).into()
            }
        }

        impl<'a,T,const N:usize> From<[&'a mut T;N]> for $dyn<'a,T> {
            fn from(arr:[&'a mut T;N]) -> Self {
                Vec::<&'a mut T>::from(arr).into()
            }
        }

        impl<'a,T,N:generic_array::ArrayLength> From<$stat<'a,T,N>> for $dyn<'a,T> {
            fn from(arr:$stat<'a,T,N>) -> Self {
                let v:Vec<&'a mut T>=<$crate::EnhancedArray<&'a mut T,N> as container_traits::IntoIter<&'a mut T>>::into_iterator(arr.0).collect();
                v.into()
            }
        }       

        impl<'a,T,const N:usize> TryInto<[&'a mut T;N]> for $dyn<'a,T> {
            type Error=Self;
            fn try_into(self) -> Result<[&'a mut T;N],Self> {
                let v:Vec<&'a mut T>=self.into();
                v.try_into()
                 .map_err(|v:Vec<&'a mut T>|v.into())
            }
        }

        impl<'a,T> $stat<'a,T,typenum::U2> {
            pub const fn new(x:&'a mut T,y:&'a mut T) -> Self {
                Self($crate::EnhancedArray::from_array([x,y]))
            }

            pub fn x(&'a self) -> &'a T { &self[0] }
            pub fn y(&'a self) -> &'a T { &self[1] }
            pub fn x_mut(&'a mut self) -> &'a mut T { & mut self[0] }
            pub fn y_mut(&'a mut self) -> &'a mut T { & mut self[1] }
        }

        impl<'a,T> $stat<'a,T,typenum::U3> {
            pub const fn new(x:&'a mut T,y:&'a mut T,z:&'a mut T) -> Self {
                Self($crate::EnhancedArray::from_array([x,y,z]))
            }

            pub fn x(&'a self) -> &'a T { &self[0] }
            pub fn y(&'a self) -> &'a T { &self[1] }
            pub fn z(&'a self) -> &'a T { &self[2] }

            pub fn x_mut(&'a mut self) -> &'a mut T { & mut self[0] }
            pub fn y_mut(&'a mut self) -> &'a mut T { & mut self[1] }
            pub fn z_mut(&'a mut self) -> &'a mut T { & mut self[2] }
        }

        paste::paste!(
            pub type [<$stat 2>]<'a,T>=$stat<'a,T,typenum::U2>;
            pub type [<$stat 3>]<'a,T>=$stat<'a,T,typenum::U3>;
            pub type [<$stat 4>]<'a,T>=$stat<'a,T,typenum::U4>;
        );

        impl<'a, T,N:generic_array::ArrayLength> Into<$crate::EnhancedArray<&'a mut T,N>> for $stat<'a,T,N> {
            fn into(self) -> $crate::EnhancedArray<&'a mut T,N> {
                self.0
            }
        }

        impl<'a, T, N:generic_array::ArrayLength> $stat<'a,T,N> {
            fn into_array<const U:usize>(self) -> [&'a mut T;U]
            where typenum::Const<U>: generic_array::IntoArrayLength<ArrayLength = N> {
                self.0
                    .into_array::<U>()
            }

            fn from_array<const U:usize>(arr:[&'a mut T;U]) -> Self
            where typenum::Const<U>: generic_array::IntoArrayLength<ArrayLength = N> {
                $crate::EnhancedArray::<&'a mut T,N>::from_array(arr).into()
            }
        }
        
        impl<'a, T, N:generic_array::ArrayLength> Into<Vec<&'a mut T>> for $stat<'a, T,N> {
            fn into(self) -> Vec<&'a mut T> {
                <$crate::EnhancedArray<&'a mut T,N> as container_traits::IntoInner>::into_inner(self.0)
                    .into_iter()
                    .collect()
            }
        }

        impl<'a, T, N:generic_array::ArrayLength> TryFrom<Vec<&'a mut T>> for $stat<'a,T,N> {
            type Error=container_traits::LenNotEqualToRequiredLenError;
            fn try_from(v:Vec<&'a mut T>) -> Result<Self,Self::Error> {
                <$crate::EnhancedArray<&'a mut T,N> as TryFrom<Vec<&'a mut T>>>::try_from(v)
                    .map(|a|Self(a))
            }
        }

        impl<'a, T, N:generic_array::ArrayLength> TryFrom<$dyn<'a,T>> for $stat<'a,T,N> {
            type Error=container_traits::LenNotEqualToRequiredLenError;
            fn try_from(v:$dyn<'a,T>) -> Result<Self,Self::Error> {
                let v:Vec<&'a mut T>=<$crate::EnhancedVec<&'a mut T> as container_traits::IntoInner>::into_inner(v.0);
                <$crate::EnhancedArray<&'a mut T,N> as TryFrom<Vec<&'a mut T>>>::try_from(v)
                    .map(|a|Self(a))
            }
        }

        impl<'a, T> Into<$crate::EnhancedVec<&'a mut T>> for $dyn<'a,T> {
            fn into(self) -> $crate::EnhancedVec<&'a mut T> {
                self.0
            }
        }


        // impl<C:'static> $gen<C> {
        //     pub fn from_unchecked_view_mut<C2:'static+Into<C>>(value:$gen<C2>) -> Self {
        //         Self(value.0.into())
        //     }

        //     pub fn into_unchecked_view_mut<C2:'static>(self) -> $gen<C2> where C:Into<C2> {
        //         $gen::from_unchecked_view_mut(self)
        //     }
        //     pub fn try_from_unchecked_view_mut<C2:'static>(value:$gen<C2>) -> Result<Self, $gen<C2>> where C2 : TryInto<C,Error=C2> {
        //         value.0
        //              .try_into()
        //              .map(|c|$gen(c))
        //              .map_err(|c2|$gen(c2))
        //     }
        //     pub fn try_into_unchecked_view_mut<C2:'static>(self) -> Result<$gen<C2>, Self> where C:TryInto<C2,Error=C> {
        //         $gen::try_from_unchecked_view_mut(self)
        //     }
        // }

    };
}