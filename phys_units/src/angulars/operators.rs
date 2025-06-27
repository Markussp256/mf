use super::generic::*;
use crate::duration::Duration_generic::Duration;

crate::physical_quantity::impl_mul_div_vars!(Duration, AngularJerk, AngularAcceleration);
crate::physical_quantity::impl_mul_div_vars!(Duration, AngularAcceleration, AngularSpeed);
crate::physical_quantity::impl_mul_div_vars!(Duration, AngularSpeed, Angle);


#[test]
fn test_mul() {
    use algebra_traits::TryDiv;
    use crate::{Angle, Duration, AngularSpeed, AngularAcceleration, AngularJerk, Degrees, Seconds, DegreesPerSecond3};
    let f=2.2;
    let a=Angle::from_deg(f);
    let s=Duration::from_sec(1.0);
    let avel:AngularSpeed=<Angle as TryDiv<Duration>>::try_div(a, s).unwrap();
    let aacc:AngularAcceleration=<AngularSpeed as TryDiv<Duration>>::try_div(avel, s).unwrap();
    let ajrk:AngularJerk=<AngularAcceleration as TryDiv<Duration>>::try_div(aacc, s).unwrap();
    assert_eq!(ajrk.deg_per_sec3(),f);
}