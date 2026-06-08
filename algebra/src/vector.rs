
// use derive_more::{From, Index, IndexMut};
// use algebra_traits::{Max, Nonnegative};
// use num_traits::Zero;

// use container_derive::{
//     ArrayTraits, Container, NormSquared, Norm, Vectorspace, VecTraits
// };


#[macro_export]
macro_rules! gen_vector {
    ($gen:ident, $dyn:ident, $stat:ident) => {
        #[derive(Clone, Debug, PartialEq,
            algebra_derive::Vector,
            container_derive::Container,
            derive_more::AsRef,
            derive_more::AsMut,
            derive_more::From,
            derive_more::Index,
            derive_more::IndexMut,
        )]
        pub struct $gen<C>(C);

        pub type $dyn<T>=$gen<$crate::EnhancedVec<T>>;
        pub type $stat<T,N:generic_array::ArrayLength>=$gen<$crate::EnhancedArray<T,N>>;

        impl<C:container_traits::ItemT<T=T>+container_traits::Iter<T>,T> $gen<C> {

            pub fn try_max<'a>(&'a self) -> Option<&'a T> where T : algebra_traits::Max {
                self.0
                    .iter()
                    .reduce(|a,b| if a > b { a } else { b })
            }

            pub fn max_norm_of_entries<SO:algebra_traits::Max+num_traits::Zero>(&self) -> algebra_traits::Nonnegative<SO>
                where T: 'static+algebra_traits::Norm<NormT=SO> {
                self.0
                    .iter()
                    .map(<T as algebra_traits::Norm>::norm)
                    .reduce(|a,b|if a > b { a } else { b})
                    .unwrap_or(<algebra_traits::Nonnegative<SO> as num_traits::Zero>::zero())
            }
        }

        impl<C:container_traits::LinearContainerView<T=T>, T:std::fmt::Display> std::fmt::Display for $gen<C> {
            container_traits::impl_display_for_linear_container!();
        }

        impl<C : algebra_traits::Norm<NormT=NT>,
             NT: algebra_traits::RealNumber> TryFrom<$gen<C>> for $crate::Unit<$gen<C>> {
            type Error=$gen<C>;
            fn try_from(value:$gen<C>) -> Result<Self, $gen<C>> {
                Self::try_new(value)
            }
        }

        impl<C:container_traits::LinearContainer> $gen<C> {
            pub fn try_into_max(self) -> Option<C::T> where C::T : algebra_traits::Max {
                self.0
                    .into_iterator()
                    .reduce(<C::T as algebra_traits::Max>::into_max)
            }

            pub fn into_max_norm_of_entries<SO:algebra_traits::Max+num_traits::Zero>(self) -> algebra_traits::Nonnegative<SO>
            where C::T: 'static+algebra_traits::Norm<NormT=SO> {
                $crate::utils::max_norm(self)
            }
        }

        impl<C:'static+$crate::IntoEnhancedContainer> $crate::IntoEnhancedContainer for $gen<C> {
            type OutputC=<C as $crate::IntoEnhancedContainer>::OutputC;
            fn into_enhanced_container(self) -> $crate::EnhancedContainer<Self::OutputC> {
                <C as $crate::IntoEnhancedContainer>::
                    into_enhanced_container(self.0)
            }
        }

        impl<C:'static> $gen<C> {
            pub fn from_unchecked<C2:'static+Into<C>>(value:$gen<C2>) -> Self {
                Self(value.0.into())
            }

            pub fn into_unchecked<C2:'static>(self) -> $gen<C2> where C:Into<C2> {
                $gen::from_unchecked(self)
            }
            pub fn try_from_unchecked<C2:'static>(value:$gen<C2>) -> Result<Self, $gen<C2>> where C2 : TryInto<C,Error=C2> {
                value.0
                     .try_into()
                     .map(|c|$gen(c))
                     .map_err(|c2|$gen(c2))
            }
            pub fn try_into_unchecked<C2:'static>(self) -> Result<$gen<C2>, Self> where C:TryInto<C2,Error=C> {
                $gen::try_from_unchecked(self)
            }
        }

        impl<T> Into<$crate::EnhancedVec<T>> for $dyn<T> {
            fn into(self) -> $crate::EnhancedVec<T> {
                self.0
            }
        }

        utils::    into_via!(impl<T>               Into<Vec<T>>            for $dyn<T>, via $crate::EnhancedVec<T>);
        utils::    from_via!(impl<T>               From<Vec<T>>            for $dyn<T>, via $crate::EnhancedVec<T>);

        impl<T,N:generic_array::ArrayLength> From<generic_array::GenericArray<T,N>> for $dyn<T> {
            fn from(value:generic_array::GenericArray<T,N>) -> Self {
                let v:Vec<T>=value.into_iter().collect();
                v.into()
            }
        }

        impl<T,N:generic_array::ArrayLength> From<$stat<T,N>> for $dyn<T> {
            fn from(value:$stat<T,N>) -> Self {
                let v:Vec<T>=value.into();
                v.into()
            }
        }

        impl<T,const N:usize> TryInto<[T;N]> for $dyn<T> {
            type Error=Self;
            fn try_into(self) -> Result<[T;N],Self> {
                let v:Vec<T>=self.into();
                v.try_into()
                 .map_err(|v:Vec<T>|v.into())
            }
        }

        impl<T> $stat<T,typenum::U2> {
            pub const fn new(x:T,y:T) -> Self {
                Self($crate::EnhancedArray::from_array([x,y]))
            }

            pub fn x(&self) -> &T { &self[0] }
            pub fn y(&self) -> &T { &self[1] }
        }

        impl<T> $stat<T,typenum::U3> {
            pub const fn new(x:T,y:T,z:T) -> Self {
                Self($crate::EnhancedArray::from_array([x,y,z]))
            }

            pub fn x(&self) -> &T { &self[0] }
            pub fn y(&self) -> &T { &self[1] }
            pub fn z(&self) -> &T { &self[2] }
        }

        impl<T : algebra_traits::ConstNonZero> algebra_traits::ConstNonZero for $stat<T,typenum::U2> {
            const NONZERO:Self=Self($crate::EnhancedArray::from_array([T::NONZERO,T::NONZERO]));
        }

        impl<T : algebra_traits::ConstNonZero> algebra_traits::ConstNonZero for $stat<T,typenum::U3> {
            const NONZERO:Self=Self($crate::EnhancedArray::from_array([T::NONZERO,T::NONZERO,T::NONZERO]));
        }

        paste::paste!(
            pub type [<$stat 2>]<T>=$stat<T,typenum::U2>;
            pub type [<$stat 3>]<T>=$stat<T,typenum::U3>;
            pub type [<$stat 4>]<T>=$stat<T,typenum::U4>;
        );

        impl<T,N:generic_array::ArrayLength> Into<$crate::EnhancedArray<T,N>> for $stat<T,N> {
            fn into(self) -> $crate::EnhancedArray<T,N> {
                self.0
            }
        }

        impl<T, N : generic_array::ArrayLength> Into<generic_array::GenericArray<T,N>> for $stat<T,N> {
            fn into(self) -> generic_array::GenericArray<T,N> {
                <Self as container_traits::IntoInner>::into_inner(self).into()
            }
        }
        impl<T, N : generic_array::ArrayLength> From<generic_array::GenericArray<T,N>> for $stat<T,N> {
            fn from(value:generic_array::GenericArray<T,N>) -> Self {
                $crate::EnhancedArray::<T,N>::from(value).into()
            }
        }
        impl<T, N : generic_array::ArrayLength> Into<Vec<T>>                           for $stat<T,N> {
            fn into(self) -> Vec<T> {
                <Self as container_traits::IntoInner>::into_inner(self).into()
            }
        }
        impl<T, N : generic_array::ArrayLength> TryFrom<Vec<T>>                        for $stat<T,N> {
            type Error=container_traits::LenNotEqualToRequiredLenError;
            fn try_from(value:Vec<T>) -> Result<Self,container_traits::LenNotEqualToRequiredLenError> {
                container_traits::LenNotEqualToRequiredLenError::try_new(N::to_usize(),value.len())?;
                Ok(generic_array::GenericArray::<T,N>::try_from_iter(value.into_iter()).unwrap().into())
            }
        }
        impl<T, N : generic_array::ArrayLength> TryFrom<$dyn<T>>                       for $stat<T,N> {
            type Error=container_traits::LenNotEqualToRequiredLenError;
            fn try_from(value:$dyn<T>) -> Result<Self,container_traits::LenNotEqualToRequiredLenError> {
                let len=<$dyn<T> as container_traits::Len>::len(&value);
                container_traits::LenNotEqualToRequiredLenError::try_new(N::to_usize(),len)?;
                Ok(generic_array::GenericArray::<T,N>::try_from_iter(value.into_iter()).unwrap().into())
            }
        }
    };
}

#[macro_export]
macro_rules! gen_vector_and_view {
    ($name:ident) => {
        paste::paste!(
            $crate::gen_vector!([<$name Generic>],[<$name Dyn>],$name);
            $crate::gen_vector_view!([<$name ViewGeneric>],[<$name ViewDyn>],[<$name View>]);
            $crate::gen_vector_view_mut!([<$name ViewMutGeneric>],[<$name ViewMutDyn>],[<$name ViewMut>]);

            impl<T,CS:container_traits::LinearContainer<T=T>
                     +container_traits::Rebind<container_traits::LinearContainerConstructError>>
                      container_traits::ContainerViewable<usize> for [<$name Generic>]<CS> {
                type Viewer<'a>=[<$name ViewGeneric>]<<CS as container_traits::Rebind<container_traits::LinearContainerConstructError>>::With<&'a T>> where CS : 'a, T:'a;
                fn as_view<'a>(&'a self) -> Self::Viewer<'a> {
                    <Self::Viewer<'a> as container_traits::AnyFromIterator<&'a T,container_traits::LinearContainerConstructError>>::any_from_iter(
                        None,
                        <Self as container_traits::Iter<T>>::iter(self)
                    ).unwrap()
                }
            }

            impl<T,
                 CS : container_traits::LinearContainer<T=T>
                      +container_traits::LinearContainerViewMut<T=T> 
                      +container_traits::Rebind<container_traits::LinearContainerConstructError>>
                    container_traits::ContainerViewMutable<usize> for [<$name Generic>]<CS> {
                type ViewMuter<'a>=[<$name ViewGeneric>]<<CS as container_traits::Rebind<container_traits::LinearContainerConstructError>>::With<&'a mut T>> where CS: 'a, T:'a;
                fn as_view_mut<'a>(&'a mut self) -> Self::ViewMuter<'a> {
                    <Self::ViewMuter<'a> as container_traits::AnyFromIterator<&'a mut T,container_traits::LinearContainerConstructError>>::any_from_iter(
                        None,
                        <Self as container_traits::IterMut<T>>::iter_mut(self)
                    ).unwrap()
                }
            }
        );
    }
}

gen_vector_and_view!(Vector);

fn test_is_viewer<'a,N:generic_array::ArrayLength>(v:&'a Vector<f64,N>) -> VectorView<'a,f64,N> {
    <Vector<f64,N> as container_traits::ContainerViewable<usize>>::as_view(v)
}

fn test<'a>(a:Vector3<f64>) -> impl Sized+container_traits::ContainerViewable<usize,Viewer<'a>=VectorView3<'a,f64>> {
    a
}