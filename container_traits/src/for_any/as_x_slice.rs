macro_rules! as_x_slice {
    ($tr:ident,$fn:ident $(,$m:ident)?) => {
        pub trait $tr {
            type T;
            fn $fn(& $($m)? self) -> & $($m)? [Self::T];
        }
        
        impl<T> $tr for Vec<T> {
            type T=T;
            fn $fn(& $($m)? self) -> & $($m)? [T] {
                self.$fn()
            }
        }
        
        impl<T,const N:usize> $tr for [T;N] {
            type T=T;
            fn $fn(& $($m)? self) -> & $($m)? [T] {
                self.$fn()
            }
        }
    };
}
as_x_slice!(AsSlice,as_slice);
as_x_slice!(AsMutSlice,as_mut_slice, mut);
