crate::unit_trait!(RadiansPerSecond3, rad_per_sec3, RAD_PER_SEC3);
crate::unit_trait!(DegreesPerSecond3, deg_per_sec3, DEG_PER_SEC3);

/// construct pairs of angular jerk types 
/// # Example
///
/// ```rust
/// phys_units::gen_angular_jerk_types!(
///     AngularJerkMeasure0, AngularJerk0,
///     AngularJerkMeasure1, AngularJerk1);
/// use phys_units::DegreesPerSecond3;
/// let a=AngularJerkMeasure0::from_deg_per_sec3(42.0);
/// assert_eq!(a.deg_per_sec3(), 42.0);
/// let da=AngularJerk0::from_deg_per_sec3(3.0);
/// let sum:AngularJerkMeasure0=a+da;
/// assert_eq!(sum.deg_per_sec3(),42.0+3.0);
/// ```
#[macro_export]
macro_rules! gen_angular_jerk_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(RAD_PER_SEC3,
            DEG_PER_SEC3 = RAD_PER_SEC3 *std::f64::consts::PI / 180.0);

       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            RadiansPerSecond3, rad_per_sec3, RAD_PER_SEC3,
            DegreesPerSecond3, deg_per_sec3, DEG_PER_SEC3);
    

        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, RadiansPerSecond3, rad_per_sec3, ANGL_DIM=1, TIME_DIM=-3);
        );
        )*
    };
}
crate::gen_angular_jerk_types!(AngularJerkMeasure, AngularJerk);