macro_rules! as_x_slice {
    ($tr:ident,$fn:ident $(,$m:ident)?) => {
        pub trait $tr<T> {
            fn $fn(& $($m)? self) -> & $($m)? [T];
        }
        
        impl<T> $tr<T> for Vec<T> {
            fn $fn(& $($m)? self) -> & $($m)? [T] {
                self.$fn()
            }
        }
        
        impl<T,const N:usize> $tr<T> for [T;N] {
            fn $fn(& $($m)? self) -> & $($m)? [T] {
                self.$fn()
            }
        }
    };
}
as_x_slice!(AsSlice,as_slice);
as_x_slice!(AsMutSlice,as_mut_slice, mut);
