
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

        impl<C:container_traits::LinearContainer> $gen<C> {
            pub fn try_max(self) -> Option<C::T> where C::T : algebra_traits::Max {
                self.0
                    .into_iterator()
                    .reduce(<C::T as algebra_traits::Max>::into_max)
            }

            pub fn max_norm_of_entries<SO:algebra_traits::Max+num_traits::Zero>(self) -> algebra_traits::Nonnegative<SO>
            where C::T: 'static+algebra_traits::Norm<NormT=SO> {
                $crate::utils::max_norm(self)
            }
        }

        impl<C:container_traits::LinearContainer<T=T>, T:std::fmt::Display> std::fmt::Display for $gen<C> {
            container_traits::impl_display_for_linear_container!();
        }

        impl<C:'static+$crate::IntoEnhancedContainer> $crate::IntoEnhancedContainer for $gen<C> {
            type OutputC=<C as $crate::IntoEnhancedContainer>::OutputC;
            fn into_enh_cont(self) -> $crate::EnhancedContainer<Self::OutputC> {
                <C as $crate::IntoEnhancedContainer>::
                    into_enh_cont(self.0)
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

        impl<C : Clone+algebra_traits::Norm<NormT=NT>,
             NT: algebra_traits::RealNumber> TryFrom<$gen<C>> for $crate::Unit<$gen<C>> {
            type Error=$gen<C>;
            fn try_from(value:$gen<C>) -> Result<Self, $gen<C>> {
                Self::try_new(value)
            }
        }

        pub type $dyn<T>=$gen<$crate::EnhancedVec<T>>;

        impl<T> Into<$crate::EnhancedVec<T>> for $dyn<T> {
            fn into(self) -> $crate::EnhancedVec<T> {
                self.0
            }
        }

        utils::    into_via!(impl<T>               Into<Vec<T>>      for $dyn<T>, via $crate::EnhancedVec<T>);
        utils::    from_via!(impl<T>               From<Vec<T>>      for $dyn<T>, via $crate::EnhancedVec<T>);
        // utils::try_into_via!(impl<T,const N:usize> TryInto<[T;N]>    for $dyn<T>, via $stat<T,N>);
        utils::    from_via!(impl<T,const N:usize> From<[T;N]>       for $dyn<T>, via Vec<T>);
        utils::    from_via!(impl<T,const N:usize> From<$stat<T,N>>  for $dyn<T>, via Vec<T>);
        
        impl<T,const N:usize> TryInto<[T;N]> for $dyn<T> {
            type Error=Self;
            fn try_into(self) -> Result<[T;N],Self> {
                let v:Vec<T>=self.into();
                v.try_into()
                 .map_err(|v:Vec<T>|v.into())
            }
        }

        pub type $stat<T,const N:usize>=$gen<$crate::EnhancedArray<T,N>>;
        
        impl<T> $stat<T,2> {
            pub const fn new(x:T,y:T) -> Self {
                Self($crate::EnhancedArray::new([x,y]))
            }

            pub fn x(&self) -> &T { &self[0] }
            pub fn y(&self) -> &T { &self[1] }
        }

        impl<T> $stat<T,3> {
            pub const fn new(x:T,y:T,z:T) -> Self {
                Self($crate::EnhancedArray::new([x,y,z]))
            }

            pub fn x(&self) -> &T { &self[0] }
            pub fn y(&self) -> &T { &self[1] }
            pub fn z(&self) -> &T { &self[2] }
        }

        impl<T : algebra_traits::ConstNonZero> algebra_traits::ConstNonZero for $stat<T,2> {
            const NONZERO:Self=Self($crate::EnhancedArray::new([T::NONZERO,T::NONZERO]));
        }

        impl<T : algebra_traits::ConstNonZero> algebra_traits::ConstNonZero for $stat<T,3> {
            const NONZERO:Self=Self($crate::EnhancedArray::new([T::NONZERO,T::NONZERO,T::NONZERO]));
        }

        paste::paste!(
            pub type [<$stat 2>]<T>=$stat<T,2>;
            pub type [<$stat 3>]<T>=$stat<T,3>;
            pub type [<$stat 4>]<T>=$stat<T,4>;
        );

        impl<T,const N:usize> Into<$crate::EnhancedArray<T,N>> for $stat<T,N> {
            fn into(self) -> $crate::EnhancedArray<T,N> {
                self.0
            }
        }
        
        utils::    into_via!(impl<T, const N:usize> Into<[T;N]>      for $stat<T,N>, via $crate::EnhancedArray<T,N>);
        utils::    from_via!(impl<T, const N:usize> From<[T;N]>      for $stat<T,N>, via $crate::EnhancedArray<T,N>);
        utils::    into_via!(impl<T, const N:usize> Into<Vec<T>>     for $stat<T,N>, via [T;N]);
        utils::try_from_via!(impl<T, const N:usize> TryFrom<Vec<T>>  for $stat<T,N>, via [T;N]);
        utils::try_from_via!(impl<T, const N:usize> TryFrom<$dyn<T>> for $stat<T,N>, via [T;N]);
    };
}
gen_vector!(VectorGeneric, VectorDyn, Vector);