// crate::phys_quant!(positional    typename: AngularPosition,
//                    differential  typename: Angle,
//                    dimensions: 1, 0, 0, 0, 0, 0,
//                    4measurement_support: Angle, radians,
//                    si-unit: Radians, rad);



// here Traits and macros are defined that can be used for any physical quantity

// note that with PhysicalQuantity wie could implement clone by
// clone(&self) -> Self { Self::from_si(self.si()) }
// hence it does not make sense to implement PhysicalQuantity but not Clone

pub trait PhysicalQuantity<F>{
    const ANGL_DIM: i32;
    const TIME_DIM: i32;
    const LENG_DIM: i32;
    const MASS_DIM:i32;
    const CURR_DIM:i32;
    const TEMP_DIM:i32;

    fn si(self) -> F;
    fn from_si(f: F) -> Self;

    // provided method
    fn dims() -> [i32; 6] {
        [Self::ANGL_DIM,
         Self::TIME_DIM,
         Self::LENG_DIM,
         Self::MASS_DIM,
         Self::CURR_DIM,
         Self::TEMP_DIM]
    }
}

macro_rules! impl_mul {
    ($A:ident, $B:ident, $Res:ident) => {
        impl<F:num_traits::One> std::ops::Mul<$B<F>> for $A<F>
        where $Res<F> : TryFrom<$crate::GenericPhysQuant<F>>,
        $crate::GenericPhysQuant<F> : From<Self>+From<$B<F>> {
            type Output = $Res<F>;
            fn mul(self, b: $B<F>) -> Self::Output {
                Self::Output::try_from(
                    $crate::GenericPhysQuant::from(self)
                        * $crate::GenericPhysQuant::from(b),
                ).ok().unwrap()
            }
        }
    };
}
pub (crate) use impl_mul;

macro_rules! impl_div {
    ($A:ident, $B:ident, $Res:ident) => {
        impl<F:algebra_traits::TryDiv<Output=F>> std::ops::Div<algebra_traits::NonZero<$B<F>>> for $A<F>
        where $Res<F> : TryFrom<$crate::GenericPhysQuant<F>>,
        Self : Into<$crate::GenericPhysQuant<F>>,
        $B<F>: Into<$crate::GenericPhysQuant<F>> {
            type Output = $Res<F>;
            fn div(self, b: algebra_traits::NonZero<$B<F>>) -> Self::Output {
                let gpq=<$crate::GenericPhysQuant<F> as algebra_traits::TryDiv>::try_div(
                        self.into(),
                        b.into_inner().into()).ok().unwrap();
                Self::Output::try_from(gpq).ok().unwrap()
            }
        }
    };
}
pub (crate) use impl_div;


macro_rules! impl_try_div {
    ($A:ident, $B:ident, $Res:ident) => {
        impl<F:algebra_traits::TryDiv<Output=F,Error=E>,E : From<algebra_traits::DivisionByZeroError>> algebra_traits::TryDiv<$B<F>> for $A<F>
        where $Res<F> : TryFrom<$crate::GenericPhysQuant<F>>,
              $B  <F> : TryFrom<$crate::GenericPhysQuant<F>>+algebra_traits::IsAZero,
              algebra_traits::DivisionByZeroError : Into<E>,
        $crate::GenericPhysQuant<F> : From<Self> + From<$B<F>> {
            type Output = $Res<F>;
            type Error=E;
            fn is_divable_by(&self, b: &$B<F>) -> Result<(), E> {
                algebra_traits::DivisionByZeroError::try_new(b)?;
                Ok(())
            }

            fn try_div(self, b: $B<F>) -> Result<Self::Output, E> {
                let gpq=$crate::GenericPhysQuant::from(self).try_div(
                        $crate::GenericPhysQuant::from(b))?;
                Ok(Self::Output::try_from(gpq).ok().unwrap())
            }
        }
    };
}
pub (crate) use impl_try_div;

macro_rules! impl_pow2 {
    ($A:ident, $A2:ident) => {
        impl<F:algebra_traits::Pow2> algebra_traits::Pow2 for $A<F>
        where $crate::GenericPhysQuant<F> : algebra_traits::Pow2<Output=$crate::GenericPhysQuant<F>>,
        $A2<F> : TryFrom<$crate::GenericPhysQuant<F>>,
        Self : Into<$crate::GenericPhysQuant<F>> {
            type Output=$A2<F>;
            fn pow2(self) -> Self::Output {
                Self::Output::try_from(
                    <$crate::GenericPhysQuant<F> as algebra_traits::Pow2>::pow2(self.into())
                ).ok().unwrap()
            }
        }
    };
}
pub (crate) use impl_pow2;

macro_rules! impl_try_sqrt {
    ($A:ident, $sqrtA:ident) => {
        impl<F,E : From<algebra_traits::SqrtError>> algebra_traits::TrySqrt for $A<F> where
        $crate::GenericPhysQuant<F> : algebra_traits::TrySqrt<
            Output = $crate::GenericPhysQuant<F>,
            Error  = either::Either<$crate::generic_physical_quantity::FromAQuantityWithAnOddDimensionWeCanNotTakeSquareroot,E>>,
        $sqrtA<F> : TryFrom<$crate::GenericPhysQuant<F>>,
        Self : Into<$crate::GenericPhysQuant<F>>+algebra_traits::ConstZero+PartialOrd {
            type Output=$sqrtA<F>;
            type Error=E;

            fn is_sqrtable(&self) -> Result<(),E> {
                if self < &<Self as algebra_traits::ConstZero>::ZERO {
                    Err(algebra_traits::SqrtError::SqrtOfNegativeNumberNotPossible.into())
                } else {
                    Ok(())
                }
            }

            fn try_sqrt(self) -> Result<Self::Output, E> {
                self.is_sqrtable()?;
                let pq:$crate::GenericPhysQuant<F>=<$crate::GenericPhysQuant<F> as algebra_traits::TrySqrt>::try_sqrt(
                            self.into())
                    .map_err(|e|e.right().unwrap())?;
                Ok($sqrtA::try_from(pq).ok().unwrap())
            }
        }
    };
}
pub (crate) use impl_try_sqrt;

// macro_rules! impl_try_sqrt {
//     ($A:ident, $SqrtA:ident) => {
//         impl algebra_traits::TrySqrt for $A {
//             type Output=$SqrtA;
//             fn try_sqrt(self) -> Option<Self::Output> {
//                 $crate::GenericPhysQuant::from(self)
//                         .try_sqrt()
//                         .map(|pq|$SqrtA::try_from(pq).unwrap())
//             }
//         }
//     };
// }
// pub (crate) use impl_try_sqrt;

macro_rules! impl_mul_div_vars {
    ($A:ident, $B:ident, $Res:ident) => {
        $crate::physical_quantity::impl_mul!($A, $B, $Res);
        $crate::physical_quantity::impl_mul!($B, $A, $Res);

        $crate::physical_quantity::impl_try_div!($Res, $A, $B);
        $crate::physical_quantity::impl_try_div!($Res, $B, $A);
        $crate::physical_quantity::impl_div!($Res, $A, $B);
        $crate::physical_quantity::impl_div!($Res, $B, $A);
    };
}
pub (crate) use impl_mul_div_vars;

macro_rules! impl_pow2_sqrt {
    ($A:ident, $A2:ident) => {
        $crate::physical_quantity::impl_mul!($A, $A, $A2);
        $crate::physical_quantity::impl_try_div!($A2, $A, $A);
        $crate::physical_quantity::impl_div!($A2, $A, $A);
        $crate::physical_quantity::impl_pow2!($A, $A2);
        $crate::physical_quantity::impl_try_sqrt!($A2, $A);
    }
}
pub (crate) use impl_pow2_sqrt;

/// implements the trait PhysicalQuantity
/// 
/// if existent also conversion from and to the corresponding type in crate measurements.
#[macro_export]
macro_rules! impl_PhysQuant {
    ($t:ty, $si_tr_name:ident, $si_name:ident
        $(,ANGL_DIM=$angl_dim:literal)?
        $(,TIME_DIM=$time_dim:literal)?
        $(,LENG_DIM=$leng_dim:literal)?
        $(,MASS_DIM=$mass_dim:literal)?
        $(,CURR_DIM=$curr_dim:literal)?
        $(,TEMP_DIM=$temp_dim:literal)?) => {

        impl<F> $crate::PhysicalQuantity<F> for $t where $t : $crate::$si_tr_name<F> {
            const ANGL_DIM:i32 = 0$(+$angl_dim)?;
            const TIME_DIM:i32 = 0$(+$time_dim)?;
            const LENG_DIM:i32 = 0$(+$leng_dim)?;
            const MASS_DIM:i32 = 0$(+$mass_dim)?;
            const CURR_DIM:i32 = 0$(+$curr_dim)?;
            const TEMP_DIM:i32 = 0$(+$temp_dim)?;
            fn si(self) -> F {
                <Self as $crate::$si_tr_name<F>>::$si_name(self)
            }

            fn from_si(f: F) -> Self {
                paste::paste!(
                    <Self as $crate::$si_tr_name<F>>::[<from_ $si_name>](f)
                )
            }
        }
    };
}

#[macro_export]
macro_rules! conv_meas {
    ($t:ty, $si_tr_name:ident, $si_name:ident, $meas_name:ident, $meas_si:ident) => {
        paste::paste!(
            #[cfg(feature = "measurements_support")]
            impl From<measurements::$meas_name> for $t {
                fn from(value: measurements::$meas_name) -> Self {
                    <Self as $crate::$si_tr_name<f64>>::[<from_ $si_name>](value.[<as_ $meas_si>]())
                }
            }
    
            #[cfg(feature = "measurements_support")]
            impl Into<measurements::$meas_name> for $t {
                fn into(self) -> measurements::$meas_name {
                    measurements::$meas_name::[<from_ $meas_si>](
                        <Self as $crate::$si_tr_name<f64>>::$si_name(self))
                }
            });
    };
}

#[macro_export]
macro_rules! conv_simple_si_units {
    ($t:ty, $si_tr_name:ident, $si_name:ident, $simple_ty:ty, $simple_si:ident) => {
        paste::paste!(
            #[cfg(feature = "simple_si_units_support")]
            impl From<$simple_ty> for $t {
                fn from(value: $simple_ty) -> Self {
                    <Self as $crate::$si_tr_name<f64>>::[<from_ $si_name>](value.[<to_ $simple_si>]())
                }
            }
    
            #[cfg(feature = "simple_si_units_support")]
            impl Into<$simple_ty> for $t {
                fn into(self) -> $simple_ty {
                    $simple_ty::[<from_ $simple_si>](
                        <Self as $crate::$si_tr_name<f64>>::$si_name(self))
                }
            });
    };
}

// requirements:
// - the traits trait, dtrait, from_as be defined using the macro [crate::quant_traits]!
// - physical dimensions
// - name of corresponding type in crate measurements if it exists
// #[macro_export]
// macro_rules! gen_PhysQuant_and_DPhysQuant {
//     ($name:ident  : $trait:path,
//      $dname:ident : $dtrait:path, 
//      $from_as:path,
//      $si_tr_name:ident,  $si_name:ident,
//      $($meas_name:ident, $meas_si:ident,)?
//      $length_dim:literal, $angle_dim:literal, $time_dim:literal,
//      $($unit_trait_path:path),*) => {

//         $crate::gen_Quant_and_DQuant!($name  :  $trait,
//                                       $dname : $dtrait,
//                                       $from_as,
//                                       $($unit_trait_path),*);
//         $crate::impl_PhysQuant!($name,  $si_tr_name, $si_name, $($meas_name, $meas_si,)? $length_dim, $angle_dim, $time_dim);
//         $crate::impl_PhysQuant!($dname, $si_tr_name, $si_name, $($meas_name, $meas_si,)? $length_dim, $angle_dim, $time_dim);
//     };
// }
