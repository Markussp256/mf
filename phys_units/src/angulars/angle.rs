pub mod angle_units {
    crate::unit_trait!(Radians,       rad,  RAD);
    crate::unit_trait!(MilliRadians, mrad, MRAD);
    crate::unit_trait!(Degrees,       deg,  DEG);
    crate::unit_trait!(Millidegrees, mdeg, MDEG);
    crate::unit_trait!(Revolutions,   rev,  REV);
}
pub use angle_units::*;

/// construct pairs of angle types 
/// # Example
///
/// ```rust
/// phys_units::gen_angle_types!(
///     AbsA,DifA,
///     AbsB,DifB,
///     AbsC,DifC);
/// use phys_units::{Degrees, Radians};
/// let a=AbsA::from_deg(42.0);
/// assert_eq!(a.deg(), 42.0);
/// assert_eq!(a.rad(), 42.0*std::f64::consts::PI / 180_f64);
/// let da=DifA::from_deg(3.0);
/// let sum:AbsA=a+da;
/// assert_eq!(sum.deg(),42.0+3.0);
/// ```
#[macro_export]
macro_rules! gen_angle_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(RAD,
            MRAD = RAD * 1e-3,
             DEG = RAD * std::f64::consts::PI / 180_f64,
            MDEG = DEG * 1e-3,
             REV = DEG * 360.0);
       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            Radians,       rad,  RAD,
            MilliRadians, mrad, MRAD,
            Degrees,       deg,  DEG,
            Millidegrees, mdeg, MDEG,
            Revolutions,   rev,  REV);
        paste::paste!(
        $crate::impls_for_angle_and_angle_pos!([<$dname _generic>]::$pname<F>);
        $crate::impls_for_angle_and_angle_pos!([<$dname _generic>]::$dname<F>);
        );

        $crate::conv_meas!($pname, Radians, rad, Angle, radians);
        $crate::conv_meas!($dname, Radians, rad, Angle, radians);
        
        $crate::conv_simple_si_units!($pname, Radians, rad, simple_si_units::geometry::Angle::<f64>, radians);
        $crate::conv_simple_si_units!($dname, Radians, rad, simple_si_units::geometry::Angle::<f64>, radians);

        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, Radians, rad, ANGL_DIM=1);
        );
        )*
    };
}

gen_angle_types!(AngularPosition, Angle);

#[macro_export]
macro_rules! impl_trigonometric_functions_for_angle {
    ($t:ty) => { 
        impl<F:algebra_traits::RealNumber> algebra_traits::scalar::trigonometric_functions::TrigonometricFunctions for $t
               where Self : $crate::Radians<F> {
            type Output=F;
            fn sin(self) -> F { <Self as $crate::Radians<F>>::rad(self).sin() }
            fn cos(self) -> F { <Self as $crate::Radians<F>>::rad(self).cos() }
            fn tan(self) -> Result<F,algebra_traits::DivError> {
                <Self as $crate::Radians<F>>::rad(self).tan()
            }
        }
    }
}

#[macro_export]
macro_rules! impls_for_angle_and_angle_pos {
    ($t:ty) => {

        impl<F:algebra_traits::RealNumber> algebra_traits::Sinc for $t where $t : $crate::Radians<F> {
            type Output=F;
            fn denominator(self) -> F {
                <Self as $crate::Radians<F>>::rad(self)
            }
        }

        $crate::impl_trigonometric_functions_for_angle!($t);

        #[cfg(feature = "cgmath_support")]
        impl<F:algebra_traits::RealNumber> From<cgmath::Rad<F>> for $t where $t : $crate::Radians<F> {
            fn from(value: cgmath::Rad<F>) -> Self {
                Self::from_rad(value.0)
            }
        }

        #[cfg(feature = "cgmath_support")]
        impl<F:algebra_traits::RealNumber> Into<cgmath::Rad<F>> for $t where $t : $crate::Radians<F> {
            fn into(self) -> cgmath::Rad<F> {
                cgmath::Rad::<F>(self.rad())
            }
        }

    }
}

#[test]
fn test_rad_2deg() {
    use algebra_traits::Tolerance;
    assert!(Angle::from_rad(1.0).rad().is_close_to_one());
    assert!(Angle::from_deg(1.0).deg().is_close_to_one());
    let res=Angle::from_deg(1.0).rad()-std::f64::consts::PI/180.0;
    println!("{res}");
    assert!(res.is_small());
    let res=Angle::from_rad(1.0).deg()-180.0/std::f64::consts::PI;
    println!("{res}");
    assert!(res.is_small());
}
#[test]
fn test_sinc() {
    use algebra_traits::scalar::trigonometric_functions::Sinc;
    for z in vec![1e-100 as f64, 1e-10, 1e-2, 10.0, -10.0, -1e-10, -1e-100] {
        assert!((Angle::from_rad(z).sinc() * z - z.sin()).abs() < 1e-15);
    }
}
