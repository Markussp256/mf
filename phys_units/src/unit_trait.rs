
#[macro_export]
macro_rules! unit_trait {
    ($unit_trait_name:ident, $unit_fn_name:ident, $const_name:ident) => {
        pub trait $unit_trait_name<F> {
            paste::paste!(
                fn [<from_ $unit_fn_name>](f:F) -> Self;
            );
            fn $unit_fn_name(self) -> F;
            const $const_name:Self;
        }
    }
}