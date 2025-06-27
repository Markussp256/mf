// use core::hash::Hash;
// use std::collections::HashMap;
// use strum::IntoEnumIterator;

#[macro_export]
macro_rules! enum_map {
    ($name:ident{ $($item:ident),*}) => {
        #[interoptopus::ffi_type]
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $($item),*
        }

        impl $name {
            pub fn iter() -> impl Iterator<Item=$name> {
                vec![$($name::$item),*].into_iter()
            }
        }

        paste::paste!(
            #[allow(non_snake_case)]
            pub mod [<$name 2 TMod>] { // the module is s.t. allow(non_snake_case) to also be applied to setters of builder
                use super::$name;
                #[derive(Clone, Debug)]
                pub struct [<$name 2 T>]<T> {
                   $($item:T),*
                }
                impl<T:Clone> From<T> for [<$name 2 T>]<T> {
                    fn from(value:T) -> Self {
                        Self{$($item:value.clone()),*}
                    }
                }
                impl<T:Default> Default for [<$name 2 T>]<T> {
                    fn default() -> Self {
                        Self{$($item:T::default()),*}
                    }
                }
                impl<T> std::ops::Index<$name> for [<$name 2 T>]<T> {
                    type Output=T;
                    fn index(&self, index:$name) -> &T {
                        match index {
                            $($name::$item => &self.$item),*
                        }
                    }
                }
                impl<T> std::ops::IndexMut<$name> for [<$name 2 T>]<T> {
                    fn index_mut(&mut self, index:$name) -> & mut T {
                        match index {
                            $($name::$item => & mut self.$item),*
                        }
                    }
                }
                impl<T> [<$name 2 T>]<T> {
                    pub fn map<T2>(self,f:impl Fn(T)-> T2) -> [<$name 2 T>]<T2> {
                        [<$name 2 T>]{$($item:f(self.$item)),*}
                    }
                }
                pub struct [<$name TryMapErr>]<E> {
                    pub item:$name,
                    pub err:E
                }
                impl<T> [<$name 2 T>]<T> {
                    pub fn try_map<T2,E>(self,f:impl Fn(T)-> Result<T2,E>) -> Result<[<$name 2 T>]<T2>,[<$name TryMapErr>]<E>>  {
                        $(let $item:T2=f(self.$item).map_err(|err|[<$name TryMapErr>]{item:$name::$item,err})?;)*
                        Ok([<$name 2 T>]{$($item),*})
                    }
                }
            }
            pub use [<$name 2 TMod>]::{[<$name 2 T>],[<$name TryMapErr>]};
        );
    };
}