crate::unit_trait!(RadiansPerSecond, rad_per_sec, RAD_PER_SEC);
crate::unit_trait!(DegreesPerSecond, deg_per_sec, DEG_PER_SEC);

/// construct pairs of angular speed types 
/// # Example
///
/// ```rust
/// phys_units::gen_angular_speed_types!(
///     AngularSpeedMeasure0, AngularSpeed0,
///     AngularSpeedMeasure1, AngularSpeed1);
/// use phys_units::{DegreesPerSecond};
/// let a=AngularSpeedMeasure0::from_deg_per_sec(42.0);
/// assert_eq!(a.deg_per_sec(), 42.0);
/// let da=AngularSpeed0::from_deg_per_sec(3.0);
/// let sum:AngularSpeedMeasure0=a+da;
/// assert_eq!(sum.deg_per_sec(),42.0+3.0);
/// ```
#[macro_export]
macro_rules! gen_angular_speed_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(RAD_PER_SEC,
                                  DEG_PER_SEC = RAD_PER_SEC * std::f64::consts::PI / 180.0);
       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            RadiansPerSecond, rad_per_sec, RAD_PER_SEC,
            DegreesPerSecond, deg_per_sec, DEG_PER_SEC);

        $crate::conv_meas!($pname, RadiansPerSecond, rad_per_sec, AngularVelocity, radians_per_second);
        $crate::conv_meas!($dname, RadiansPerSecond, rad_per_sec, AngularVelocity, radians_per_second);

        $crate::conv_simple_si_units!($pname,  RadiansPerSecond, rad_per_sec, simple_si_units::mechanical::AngularVelocity::<f64>, radians_per_second);
        $crate::conv_simple_si_units!($dname,  RadiansPerSecond, rad_per_sec, simple_si_units::mechanical::AngularVelocity::<f64>, radians_per_second);
    
        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, RadiansPerSecond, rad_per_sec, ANGL_DIM=1, TIME_DIM=-1);
        );
        )*
    };
}

crate::gen_angular_speed_types!(AngularSpeedMeasure, AngularSpeed);



#[test]
fn test_ang_speed_times_dur() {
    use crate::{RadiansPerSecond, Seconds, Radians};
    let asp = AngularSpeed::from_rad_per_sec(1.2);
    let d = crate::Duration::from_sec(1.3);
    let a = asp * d;
    assert_eq!(a.rad(), 1.56);
}

#[test]
fn test_ang_speed_div_dur() {
    use algebra_traits::TryDiv;
    use crate::{Seconds, Duration, RadiansPerSecond, AngularAcceleration, RadiansPerSecond2};
    let asp = AngularSpeed::from_rad_per_sec(1.2);
    let d:Duration = crate::Duration::from_sec(1.3);
    let a:AngularAcceleration = <AngularSpeed as TryDiv<Duration>>::try_div(asp, d).unwrap();
    assert_eq!(a.rad_per_sec2(), 1.2 / 1.3);
}
