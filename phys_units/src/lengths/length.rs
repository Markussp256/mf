pub mod length_traits {
    crate::unit_trait!(Meters,      m,    M);
    crate::unit_trait!(Millimeters, mm,   MM);
    crate::unit_trait!(Micrometers, um,   UM);
    crate::unit_trait!(Nanometers,  nm,   NM);
    crate::unit_trait!(Inches,      inch, INCH);
}
pub use length_traits::*;

/// construct pairs of length types 
/// # Example
///
/// ```rust
/// phys_units::gen_length_types!(
///     AbsX,DifX,
///     AbsY,DifY,
///     AbsZ,DifZ);
/// use phys_units::{Meters, Millimeters};
/// let ax=AbsX::from_m(42.0);
/// assert_eq!(ax.m(), 42.0);
/// assert_eq!(ax.mm(), 42.0*1e3);
/// let dx=DifX::from_mm(3.0);
/// let ax_plus_dx:AbsX=ax+dx;
/// assert_eq!(ax_plus_dx.mm(),42.0*1e3+3.0);
/// ```
#[macro_export]
macro_rules! gen_length_types {
    ($($pname:ident, $dname:ident),*) => {
        $crate::unit_conv_consts!(M,
            MM = M * 1e-3,
            UM = M * 1e-6,
            NM = M * 1e-9,
            INCH = MM * 25.4);

       $($crate::quantity!(
            positional    typename: $pname,
            differential  typename: $dname,
            units:
            Meters,      m,    M,
            Millimeters, mm,   MM,
            Micrometers, um,   UM,
            Nanometers,  nm,   NM,
            Inches,      inch, INCH);
    
        $crate::conv_meas!($pname, Meters, m, Length, meters);
        $crate::conv_meas!($dname, Meters, m, Length, meters);

        $crate::conv_simple_si_units!($pname, Meters, m, simple_si_units::base::Distance::<f64>, meters);
        $crate::conv_simple_si_units!($dname, Meters, m, simple_si_units::base::Distance::<f64>, meters);

        paste::paste!(
            $crate::impl_PhysQuant!([<$dname _generic>]::$dname<F>, Meters, m, LENG_DIM=1);
        );
        )*
    };
}
gen_length_types!(LengthMeasure, Length);

#[test]
fn test_from_inch() {
    let _a=Length::from_inch(4.2);
}


mod atan2_impl {
    use algebra_traits::{RealNumber, TryATan2};
    use crate::generic::Angle;
    use crate::Radians;
    impl<R:RealNumber> TryATan2 for super::Length_generic::Length<R> {
        type Output=Angle<R>;
        fn try_atan2(sin:Self, cos:Self) -> Option<Self::Output> {
           <R as TryATan2>::try_atan2_generic(sin, cos)
            .map(|r| Angle::from_rad(r))
        }
    }
}

use algebra_traits::{Pow2, RealNumber, Nonnegative};
use crate::generic::Area;
impl<F:Clone+RealNumber> algebra_traits::NormSquared for Length_generic::Length<F> {
    type Norm2T = Area<F>;
    fn norm_squared(self) -> Nonnegative<Self::Norm2T> {
        Nonnegative::try_new(self.pow2()).unwrap()
    }
}


#[test]
fn test_norm_on_length() {
    use algebra_traits::Norm;
    let f=2.3; //positive float
    let a=Length::from_m(f);
    assert_eq!(a, a.norm().into_signed());
}

#[test]
fn test_norm_on_neg_length() {
    use algebra_traits::Norm;
    let f=-2.3; //positive float
    let a=Length::from_m(f);
    assert_eq!(-a, a.norm().into_signed());
}