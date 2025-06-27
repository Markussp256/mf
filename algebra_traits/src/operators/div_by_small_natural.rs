#[macro_export]
macro_rules! impl_div2to10 {
    ($macro_name:path $(, $f:ident)? ) => {
        $macro_name !(2 $(,$f)?);
        $macro_name !(3 $(,$f)?);
        $macro_name !(4 $(,$f)?);
        $macro_name !(5 $(,$f)?);
        $macro_name !(6 $(,$f)?);
        $macro_name !(7 $(,$f)?);
        $macro_name !(8 $(,$f)?);
        $macro_name !(9 $(,$f)?);
        $macro_name !(10$(,$f)?);
    };
}

#[macro_export]
macro_rules! inherit_divi {
    ($l:literal, $w:ident) => {
        paste::paste!(
        impl<F:$crate::operators::div_by_small_natural::[<Div $l>]>
               $crate::operators::div_by_small_natural::[<Div $l>] for $w<F> {
            fn [<div $l>](self) -> Self {
                Self(<F as $crate::operators::div_by_small_natural::[<Div $l>]>::[<div $l>](self.0))
            }
        });
    }
}

#[macro_export]
macro_rules! inherit_div2to10 {
    ($w:ident) => {
        $crate::impl_div2to10!($crate::inherit_divi, $w);
    }
}

macro_rules! def_impl_div_by_small_natural {
    ($l0:literal $(,$l:literal)*) => {
        paste::paste!(
        pub trait [<Div $l0>] {
            fn [<div $l0>](self) -> Self;
        }
        $(
            pub trait [<Div $l>] {
                fn [<div $l>](self) -> Self;
            }
        )*
        pub trait DivBySmallNatural : [<Div $l0>] $(+ [<Div $l>] )*  {}

        impl<T: [<Div $l0>] $(+ [<Div $l>] )*> DivBySmallNatural for T {}
        );
    }
}
def_impl_div_by_small_natural!(2,3,4,5,6,7,8,9,10);