pub mod duration_traits {
    crate::unit_trait!(Seconds, sec, SEC);
    crate::unit_trait!(Milliseconds, ms, MS);
    crate::unit_trait!(Minutes, min, MIN);
    crate::unit_trait!(Hours, hour, HOUR);
    crate::unit_trait!(Days, day, DAY);
}
pub use duration_traits::*;

/// construct pairs of duration types 
/// # Example
///
/// ```rust
/// phys_units::gen_duration_types!(
///     Time0,Duration0,
///     Time1,Duration1);
/// use phys_units::{Seconds, Minutes};
/// let t0=Time0::from_sec(42.0);
/// assert_eq!(t0.sec(), 42.0);
/// assert_eq!(t0.min(), 42.0/60.0);
/// let dt0=Duration0::from_sec(3.0);
/// let t0_plus_dt0:Time0=t0+dt0;
/// assert_eq!(t0_plus_dt0.sec(),42.0+3.0);
/// ```
#[macro_export]
macro_rules! gen_duration_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(SEC,
            MS = SEC * 1e-3,
            MIN = SEC * 60.0,
            HOUR = MIN * 60.0,
            DAY = HOUR * 24.0);
        // use $crate::{Seconds, Milliseconds, Minutes, Hours, Days};
            
       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            Seconds, sec, SEC,
            Milliseconds, ms, MS,
            Minutes, min, MIN,
            Hours, hour, HOUR,
            Days, day, DAY);
    

        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, Seconds, sec, TIME_DIM=1);
        );
        )*
    };
}
crate::gen_duration_types!(Time, Duration);