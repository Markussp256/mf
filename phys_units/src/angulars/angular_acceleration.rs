crate::unit_trait!(RadiansPerSecond2, rad_per_sec2, RAD_PER_SEC2);
crate::unit_trait!(DegreesPerSecond2, deg_per_sec2, DEG_PER_SEC2);

/// construct pairs of angular acceleration types 
/// # Example
///
/// ```rust
/// phys_units::gen_angular_acceleration_types!(
///     AngularAccelerationMeasure0, AngularAcceleration0,
///     AngularAccelerationMeasure1, AngularAcceleration1);
/// use phys_units::{DegreesPerSecond2};
/// let a=AngularAccelerationMeasure0::from_deg_per_sec2(42.0);
/// assert_eq!(a.deg_per_sec2(), 42.0);
/// let da=AngularAcceleration0::from_deg_per_sec2(3.0);
/// let sum:AngularAccelerationMeasure0=a+da;
/// assert_eq!(sum.deg_per_sec2(),42.0+3.0);
/// ```
#[macro_export]
macro_rules! gen_angular_acceleration_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(RAD_PER_SEC2,
            DEG_PER_SEC2 = RAD_PER_SEC2 * std::f64::consts::PI / 180.0);

       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            RadiansPerSecond2, rad_per_sec2, RAD_PER_SEC2,
            DegreesPerSecond2, deg_per_sec2, DEG_PER_SEC2);
    
        $crate::conv_simple_si_units!($pname,  RadiansPerSecond2, rad_per_sec2, simple_si_units::mechanical::AngularAcceleration::<f64>, radians_per_second_squared);
        $crate::conv_simple_si_units!($dname,  RadiansPerSecond2, rad_per_sec2, simple_si_units::mechanical::AngularAcceleration::<f64>, radians_per_second_squared);
    

        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, RadiansPerSecond2, rad_per_sec2, ANGL_DIM=1, TIME_DIM=-2);
        );
        )*
    };
}

crate::gen_angular_acceleration_types!(AngularAccelerationMeasure, AngularAcceleration);

