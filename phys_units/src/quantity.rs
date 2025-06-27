

#[macro_export]
macro_rules! unit_conv_consts_rec {
    ($($past:ident),*;) => {};

    ($($past:ident),* ; $new:ident = $new_other:ident * $e:expr  $(, $resnew:ident = $resnew_other:ident * $rese:expr)*) => {
        paste::paste!(
            $(const [<$new _DIV_ $past>]:f64=($e as f64) * ([<$new_other _DIV_ $past>] as f64); )*
            $(const [<$past _DIV_ $new>]:f64=(1.0 as f64) / ([<$new _DIV_ $past>] as f64); )*
              const [<$new _DIV_ $new>]:f64=1.0;
        );
        $crate::unit_conv_consts_rec!($($past,)* $new; $($resnew = $resnew_other * $rese),*);
    };
}

#[macro_export]
macro_rules! unit_conv_consts {
    ($si_const_name:ident $(,$const_name:ident = $const_name_other:ident * $e:expr)*) => {
        paste::paste!(
            const [<$si_const_name _DIV_ $si_const_name>]:f64 = 1.0;
            $crate::unit_conv_consts_rec!($si_const_name ; $($const_name = $const_name_other * $e),*);
        );
    };
}


#[macro_export]
macro_rules! unit_conv_impl {
    // copy
    ($($fn_name:ident, $const_name:ident),*) => {
        $crate::unit_conv_impl!($($fn_name, $const_name),* ; $($fn_name, $const_name),*);
    };

    // base case
    (;$($fn_name2:ident, $const_name2:ident),*) => {
    };

    // recursion
    ($fn_name0:ident, $const_name0:ident $(,$fn_name:ident, $const_name:ident)* ; $($fn_name2:ident, $const_name2:ident),*) => {
        paste::paste!(
            $(
                fn [<$fn_name0 2 $fn_name2>]<F:algebra_traits::CastFromf64>() -> F { 
                    <F as algebra_traits::CastFromf64>::from_f64([<$const_name0 _DIV_ $const_name2>])
                }
            )*
        );
        $crate::unit_conv_impl!($($fn_name, $const_name),* ; $($fn_name2, $const_name2),*);
    };
}

#[macro_export]
macro_rules! match_unit_conv_const {
    ($s:ident, $rhs:ident, $u:ident, $($long:ident, $short:ident),*) => {
        paste::paste!(
        match $rhs {
            $(
                Self::$long => Self::[<$u 2 $short>]::<F>()
            ),*
        }
        )
    };
}

// match does not go well with macro_rules therefore we use if
#[macro_export]
macro_rules! div_units {
    // copy all units
    ($s:ident, $rhs:ident; $($units:ident),*) => {
        $crate::div_units!($s, $rhs;  $($units),*; $($units),*)
    };

    // base case, if during runtime there was no match
    ($s:ident, $rhs:ident; ; $($rem:ident),*) => {
        panic!("a case seems missing");
    };

    ($s:ident, $rhs:ident; $long0:ident, $short0:ident $(,$rem0:ident)*; $($rem1:ident),*) => {
        if $s == Self::$long0 {
            return $crate::match_unit_conv_const!($s, $rhs, $short0, $($rem1),*);
        }
        $crate::div_units!($s, $rhs; $($rem0),*; $($rem1),*);
    };
}

#[macro_export]
macro_rules! base_impl {
    ($name:ident, $unit_enum:ident, $si_tr_name:ident, $si_fn_name:ident) => {

        #[interoptopus::ffi_type]
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name<F> {
            value:F,
            unit:$unit_enum
        }

        paste::paste!(
        impl<F> $name<F> {
            pub const fn new(value:F, unit:$unit_enum) -> Self {
                Self{value, unit}
            }

            pub fn into_parts(self) -> (F, $unit_enum) {
                (self.value, self.unit)
            }

            pub fn value_in(self, unit:$unit_enum) -> F where F:algebra_traits::RealNumber {
                if self.unit == unit {
                    self.value
                } else {
                    self.value * self.unit.div_in::<F>(unit)
                }
            }
        }

        $crate::impl_constants!($name, $unit_enum::$si_tr_name);

        impl<F:Clone> container_traits::for_static::NumberOfDegreesOfFreedom<F> for $name<F>{
            const NDOFS:usize=1;
        }

        impl<F:Clone+algebra_traits::RealNumber> container_traits::IntoParameter<F> for $name<F> {
            fn into_parameter(self) -> F {
                self.value_in($unit_enum::$si_tr_name)
            }
        }

        impl<F:Clone+algebra_traits::RealNumber> container_traits::FromParameter<F> for $name<F> {
            fn from_parameter(f: F) -> Self {
                Self::new(f, $unit_enum::$si_tr_name)
            }
        }

        impl<F:algebra_traits::RealNumber> container_traits::IntoIter<F> for $name<F> {
            fn into_iterator(self) -> impl ExactSizeIterator<Item=F> {
                std::iter::once(self.value_in($unit_enum::$si_tr_name))
            }
        }

        impl<F:Clone+algebra_traits::RealNumber> container_traits::TryFromIterator<F,container_traits::ContainerConstructError<usize>> for $name<F> {
            fn try_take_away<I:Iterator<Item=F>>(iter:& mut I) -> Result<Self,container_traits::ContainerConstructError<usize>> {
                <F as container_traits::TryFromIterator<F,container_traits::ContainerConstructError<usize>>>::try_take_away(iter)
                    .map(|f|Self::new(f, $unit_enum::$si_tr_name))
            }

            container_traits::try_from_iter_impl!(F);
        }

        impl<F:algebra_traits::Conjugate> algebra_traits::Conjugate for $name<F> {
            fn conjugate(self) -> Self {
                Self::new(<F as algebra_traits::Conjugate>::conjugate(self.value), self.unit)
            }
        }

        impl<F:std::fmt::Display> std::fmt::Display for $name<F> {
            fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
                let width=f.width().unwrap_or(8);
                let precision=f.precision().unwrap_or(4);
                write!(f, "{: >width$.precision$} {:?}", &self.value, &self.unit, width=width, precision=precision)
            }
        }
        
        impl<F:Clone+std::cmp::PartialEq+algebra_traits::RealNumber> std::cmp::PartialEq for $name<F> {
            fn eq(&self, rhs:&Self) -> bool {
                if self.unit == rhs.unit {
                    self.value == rhs.value
                } else {
                    self.value == rhs.clone().value_in(self.unit)
                    && rhs.value == self.clone().value_in(rhs.unit)
                }
            }
        }

        impl<F:Clone+std::cmp::PartialOrd+algebra_traits::RealNumber> std::cmp::PartialOrd for $name<F> {
            fn partial_cmp(&self, rhs:&Self) -> Option<std::cmp::Ordering> {
                if self.unit == rhs.unit {
                    self.value.partial_cmp(&rhs.value)
                } else {
                    self.value.partial_cmp(&rhs.clone().value_in(self.unit))
                }
            }
        }

        impl<F:algebra_traits::RealNumber> algebra_traits::Max for $name<F> {
            fn max<'a>(&'a self, rhs:&'a Self) -> &'a Self {
                if self > rhs {
                    self
                } else {
                    rhs
                }
            }
        }


        );
    };
}

// tr_name can be defined in user trait or phys_unit trait,
// it must be included 
#[macro_export]
macro_rules! impl_unit_traits {
    ($name:ident, $unit_enum:ident $(,$tr_name:ident, $fn_name:ident, $const_name:ident)* ) => {
        $(
        impl<F:algebra_traits::RealNumber> $tr_name<F> for $name<F> {
            paste::paste!(
            fn [<from_ $fn_name>](value:F) -> Self {
                Self::new(value, $unit_enum::$tr_name)
            });
            fn $fn_name(self) -> F {
                self.value_in($unit_enum::$tr_name)
            }
            const $const_name:Self=Self{value:F::ONE, unit:$unit_enum::$tr_name};
        })*
    }
}


#[macro_export]
macro_rules! impl_constants {
    ($name:ident, $unit:path) => {
        impl<F:algebra_traits::ConstNonZero> algebra_traits::ConstNonZero for $name<F> {
            const NONZERO:Self=Self{value:F::NONZERO, unit:$unit};
        }

        impl<F:algebra_traits::ConstZero> algebra_traits::ConstZero for $name<F> {
            const ZERO:Self=Self{value:F::ZERO, unit:$unit};
        }

        impl<F:algebra_traits::ConstElement> algebra_traits::ConstElement for $name<F> {
            const ELEMENT:Self=Self{value:F::ELEMENT, unit:$unit};
        }
    }
}

#[macro_export]
macro_rules! div_i {
    ($name:ident, $l:literal) => {
        paste::paste!(
            impl<F:algebra_traits::div_by_small_natural::[<Div $l>]>
                   algebra_traits::div_by_small_natural::[<Div $l>] for $name<F> {
                fn [<div $l>](self) -> Self {
                    Self::new(
                        <F as algebra_traits::div_by_small_natural::[<Div $l>]>::[<div $l>](self.value), self.unit)
                }
            }
        );
    };
}

#[macro_export]
macro_rules! quantity {
    (positional typename:   $pname:ident,
     differential typename: $dname:ident,
    units:
    $si_tr_name:ident, $si_fn_name:ident, $si_const_name:ident
    $(
        ,$tr_name:ident, $fn_name:ident, $const_name:ident
    )*
    ) => {

        paste::paste!(

        #[allow(non_snake_case)]
        pub mod [<$dname _generic>] {

            #[interoptopus::ffi_type]
            #[repr(C)]
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub enum [<$dname Unit>] {
                $si_tr_name
                $(, $tr_name)*
            }
    
            impl Default for [<$dname Unit>] {
                fn default() -> Self {
                    Self::$si_tr_name
                }
            }

            // use super::[<$dname Unit>];
            use $crate::{$si_tr_name $(,$tr_name)*};

            $crate::base_impl!($pname, [<$dname Unit>], $si_tr_name, $si_fn_name);
            $crate::base_impl!($dname, [<$dname Unit>], $si_tr_name, $si_fn_name);

            $crate::impl_unit_traits!($pname, [<$dname Unit>] , $si_tr_name, $si_fn_name, $si_const_name $(, $tr_name, $fn_name, $const_name)*);
            $crate::impl_unit_traits!($dname, [<$dname Unit>] , $si_tr_name, $si_fn_name, $si_const_name $(, $tr_name, $fn_name, $const_name)*);


            impl<F:std::ops::Neg<Output=F>> std::ops::Neg for $dname<F> {
                type Output = Self;
                fn neg(self) -> Self {
                    Self::new(-self.value, self.unit)
                }
            }

            impl<F:algebra_traits::RealNumber> std::ops::Add for $dname<F> {
                type Output = Self;
                fn add(self, rhs:Self) -> Self {
                    Self::new(self.value + rhs.value_in(self.unit), self.unit)
                }
            }
            
            impl<F:algebra_traits::RealNumber> std::ops::Sub for $dname<F> {
                type Output = Self;
                fn sub(self, rhs:Self) -> Self {
                    Self::new(self.value - rhs.value_in(self.unit), self.unit)
                }
            }

            impl<F:algebra_traits::RealNumber> algebra_traits::IsAZero for $dname<F> {
                fn is_a_zero(&self) -> bool {
                    <F as num_traits::Zero>::is_zero(&self.value)
                }
            }

            impl<F:algebra_traits::RealNumber> num_traits::Zero for $dname<F> {
                fn zero() -> Self {
                    Self::new(<F as num_traits::Zero>::zero(), [<$dname Unit>]::$si_tr_name)
                }
            
                fn is_zero(&self) -> bool {
                    <F as num_traits::Zero>::is_zero(&self.value)
                }
            }
            
            impl<F:algebra_traits::RealNumber> std::ops::Mul<F> for $dname<F> {
                type Output = Self;
                fn mul(self, f:F) -> Self {
                    Self::new(self.value*f, self.unit)
                }
            }
            
            impl std::ops::Mul<$dname<f64>> for f64 {
                type Output=$dname<f64>;
                fn mul(self, rhs:$dname<f64>) -> Self::Output {
                    rhs * self
                }
            }

            impl<F:algebra_traits::RealNumber> algebra_traits::TryDiv<F> for $dname<F> {
                type Output=Self;
                type Error=<F as algebra_traits::TryDiv>::Error;
                fn is_divable_by(&self, f:&F) -> Result<(),<F as algebra_traits::TryDiv>::Error> {
                    self.value
                        .is_divable_by(f)
                }

                fn try_div(self, f:F) ->Result<Self,<F as algebra_traits::TryDiv>::Error> {
                    let (value,unit)=self.into_parts();
                    value.try_div(f)
                         .map(|newval|Self::new(newval, unit))
                }
            }

            impl<F:algebra_traits::RealNumber> algebra_traits::TryDiv for $dname<F> {
                type Output=F;
                type Error=<F as algebra_traits::TryDiv>::Error;
                fn is_divable_by(&self, rhs:&Self) -> Result<(), <F as algebra_traits::TryDiv>::Error> {
                    algebra_traits::DivisionByZeroError::try_new(rhs)?;
                    Ok(())
                }

                fn try_div(self, rhs:Self) -> Result<F,<F as algebra_traits::TryDiv>::Error> {
                    let (value,unit)=self.into_parts();
                    value.try_div(rhs.value_in(unit))
                }
            }

            // impl<F:algebra_traits::RealNumber> algebra_traits::TryDiv for $dname<F> {
            //     type Output=F;
            //     fn is_divable_by(&self,rhs:&Self) -> Result<F,algebra_traits::DivError> {
            //         <Self as algebra_traits::AnyDiv>::is_divable_by(self,rhs)
            //     }

            //     fn try_div(self, rhs:Self) -> Result<F,algebra_traits::DivError> {
            //         <Self as algebra_traits::AnyDiv>::any_div(self,rhs)
            //     }
            // }


            $crate::div_i!($dname, 2);
            $crate::div_i!($dname, 3);
            $crate::div_i!($dname, 4);
            $crate::div_i!($dname, 5);
            $crate::div_i!($dname, 6);
            $crate::div_i!($dname, 7);
            $crate::div_i!($dname, 8);
            $crate::div_i!($dname, 9);
            $crate::div_i!($dname,10);
            // impl<F:algebra_traits::DivBySmallNatural> algebra_traits::DivBySmallNatural for $dname<F> {}

            impl<F:algebra_traits::RealNumber> algebra_traits::IsAZero for $pname<F> {
                fn is_a_zero(&self) -> bool {
                    <F as num_traits::Zero>::is_zero(&self.value)
                }
            }

            impl<F:algebra_traits::RealNumber> std::ops::Add<$dname<F>> for $pname<F> {
                type Output=Self;
                fn add(self, rhs:$dname<F>) -> Self {
                    Self::new(self.value + rhs.value_in(self.unit), self.unit)
                }
            }

            impl<F:algebra_traits::RealNumber> std::ops::Sub for $pname<F> {
                type Output = $dname<F>;
                fn sub(self, rhs:Self) -> Self::Output {
                    Self::Output::new(self.value-rhs.value_in(self.unit), self.unit)
                }
            }

            impl<F:algebra_traits::RealNumber> std::ops::Sub<$dname<F>> for $pname<F> {
                type Output=Self;
                fn sub(self, rhs:$dname<F>) -> Self {
                    Self::new(self.value-rhs.value_in(self.unit), self.unit)
                }
            }

            impl<F:algebra_traits::RealNumber+Clone> algebra_traits::ScalarMul<F> for $dname<F> {
                fn scalar_mul(self, rhs:&F) -> Self {
                    self * rhs.clone()
                }
            }
    
            impl<F:algebra_traits::RealNumber> algebra_traits::TryScalarDiv<F> for $dname<F> {
                type Error=<Self as algebra_traits::TryDiv<F>>::Error;
                fn try_scalar_div(self, rhs:&F) -> Result<Self,Self::Error> {
                    <Self as algebra_traits::TryDiv<F>>::try_div(self, rhs.clone())
                }
            }
    
            impl<F:algebra_traits::RealNumber> algebra_traits::TryAdd for $dname<F> {
                algebra_traits::impl_tryop_from_op_impl!(Add,add, by rhs:$dname<F>);
            }
    
            impl<F:algebra_traits::RealNumber> algebra_traits::TrySub for $dname<F> {
                algebra_traits::impl_tryop_from_op_impl!(Sub,sub, by rhs:$dname<F>);
            }

            impl<F:algebra_traits::RealNumber+algebra_traits::ConstNonZero> algebra_traits::Basis<F> for $dname<F> {
                fn basis() -> impl ExactSizeIterator<Item=Self> {
                    std::iter::once(<Self as algebra_traits::ConstNonZero>::NONZERO)
                }
            }

            impl<F:algebra_traits::RealNumber+Clone+algebra_traits::ConstNonZero> algebra_traits::FiniteDimensionalVectorspace<F,1> for $dname<F> {}
               
            impl<F:algebra_traits::RealNumber+Clone> algebra_traits::Norm for $dname<F> {
                type NormT=Self;
                fn norm(self) -> algebra_traits::Nonnegative<Self> {
                    algebra_traits::Nonnegative::try_new(
                        Self::new(self.value.norm().into_signed(), self.unit)
                    ).unwrap()
                }
            }

            impl<F:algebra_traits::RealNumber> algebra_traits::Distance for $dname<F> {
                type DistT=Self;
                fn distance(self, rhs:impl Into<Self>) -> algebra_traits::Nonnegative<Self> {
                    let rhs:Self=rhs.into();
                    <Self as algebra_traits::Norm>::norm(rhs-self)
                }
            }

            impl<F:algebra_traits::RealNumber> algebra_traits::TryDistance for $dname<F> {
                type TryDistT=Self;
                type Error=std::convert::Infallible;
                fn try_distance(self, rhs:impl Into<Self>) -> Result<algebra_traits::Nonnegative<Self>, Self::Error> {
                    Ok(<Self as algebra_traits::Distance>::distance(self,rhs))
                }
            }

            impl<F:algebra_traits::RealNumber> algebra_traits::Distance for $pname<F> {
                type DistT=$dname<F>;
                fn distance(self, rhs:impl Into<Self>) -> algebra_traits::Nonnegative<$dname<F>> {
                    let rhs:Self=rhs.into();
                    <$dname<F> as algebra_traits::Norm>::norm(rhs-self)
                }
            }

            impl<F:algebra_traits::RealNumber> algebra_traits::TryDistance for $pname<F> {
                type TryDistT=$dname<F>;
                type Error=std::convert::Infallible;
                fn try_distance(self, rhs:impl Into<Self>) -> Result<algebra_traits::Nonnegative<$dname<F>>, Self::Error> {
                    Ok(<Self as algebra_traits::Distance>::distance(self,rhs))
                }
            }

            impl<F:algebra_traits::RealNumber> algebra_traits::Tolerance for $dname<F> {
                const THRESHOLD:$dname<F>=Self::new(F::THRESHOLD,[<$dname Unit>]::$si_tr_name);
            }

            impl<F:num_traits::Zero> algebra_traits::Origin for $pname<F> {
                fn origin() -> Self {
                    Self::new(F::zero(), [<$dname Unit>]::$si_tr_name)
                }
            }

            impl<F:algebra_traits::RealNumber> algebra_traits::Torsor for $pname<F> {}


            // NonZero
            impl<F:algebra_traits::Field> algebra_traits::IntegralDomain for $dname<F> {}

            impl<F:algebra_traits::RealNumber> std::ops::Div<algebra_traits::NonZero<$dname<F>>> for $dname<F> {
                type Output=F;
                fn div(self, rhs:algebra_traits::NonZero<$dname<F>>) -> F {
                    <Self as algebra_traits::TryDiv>::try_div(self, rhs.into_inner()).ok().unwrap()
                }
            }

        }

        pub use [<$dname _generic>]::[<$dname Unit>];

        impl [<$dname Unit>] {
            $crate::unit_conv_impl!($si_fn_name, $si_const_name $(, $fn_name, $const_name)*);

            fn div_in<F:algebra_traits::RealNumber>(self, rhs:Self) -> F {
                $crate::div_units!(self, rhs;
                    $si_tr_name, $si_fn_name
                    $(, $tr_name, $fn_name)*);
            }
        }

        pub type $pname=[<$dname _generic>]::$pname<f64>;
        pub type $dname=[<$dname _generic>]::$dname<f64>;

        );
    }
}