pub mod area_traits {
    crate::unit_trait!(MetersSquared, m2, M2);
    crate::unit_trait!(MillimetersSquared, mm2, MM2);
    crate::unit_trait!(InchesSquared, inch2, INCH2);
}
pub use area_traits::*;

/// construct pairs of area types 
/// # Example
///
/// ```rust
/// phys_units::gen_area_types!(
///     AreaMeasure0,Area0,
///     AreaMeasure1,Area1);
/// use phys_units::{MetersSquared, MillimetersSquared};
/// let a0=AreaMeasure0::from_m2(42.0);
/// assert_eq!(a0.m2(), 42.0);
/// let da0=Area0::from_mm2(1.0);
/// let sum:AreaMeasure0=a0+da0;
/// assert_eq!(sum.m2(),42.0+1.0*1e-6);
/// ```
#[macro_export]
macro_rules! gen_area_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(M2,
            MM2=M2 * 1e-6,
            INCH2 = MM2 * 25.4 * 25.4);
       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            MetersSquared,      m2,    M2,
            MillimetersSquared, mm2,   MM2,
            InchesSquared,      inch2, INCH2);
    
        $crate::conv_meas!($pname, MetersSquared, m2, Area, square_meters);
        $crate::conv_meas!($dname, MetersSquared, m2, Area, square_meters);
    
        $crate::conv_simple_si_units!($pname, MetersSquared, m2, simple_si_units::geometry::Area::<f64>, m2);
        $crate::conv_simple_si_units!($dname, MetersSquared, m2, simple_si_units::geometry::Area::<f64>, m2);

        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, MetersSquared, m2, TIME_DIM=0, LENG_DIM=2);
        );
        )*
    };
}
gen_area_types!(AreaMeasure, Area);