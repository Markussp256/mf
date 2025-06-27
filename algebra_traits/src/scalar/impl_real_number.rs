use crate::*;
use paste::paste;

use num_traits::Zero;

macro_rules! impl_divi4f {
    ($l:literal, $f:ident) => {
        paste::paste!(
        impl $crate::operators::div_by_small_natural::[<Div $l>] for $f {
            fn [<div $l>](self) -> Self {
                self / ($l as $f)
            }
        });
    };
}

macro_rules! impl_un_op_f {
    ($tr:ident, $fn:ident, $e:expr, $f:ident) => {
        impl $tr for $f {
            type Output=$f;
            fn $fn(self) -> Self::Output {
                let f=$e;
                f(self)
            }
        }
        $crate::impl_tryop_from_op!($tr, $fn, $f);
    }
}

macro_rules! impl_int_and_float {
    ($f:ident) => {
        impl ConstZero for $f {
            const ZERO:$f=0 as $f;
        }

        impl ConstOne for $f {
            const ONE:$f=1 as $f;
        }

        impl ConstNonZero for $f {
            const NONZERO:$f=1 as $f;
        }
    
        impl ConstElement for $f {
            const ELEMENT:$f=1 as $f;
        }

        // alg_from_ops_num_without_inv!($f);

        impl Origin for $f {
            fn origin() -> Self {
                <$f as num_traits::Zero>::zero()
            }
        }

        impl IsAZero for $f {
            fn is_a_zero(&self) -> bool {
                self.is_zero()
            }
        }

        impl Conjugate for $f {
            fn conjugate(self) -> Self {
                 self
            }
        }

        impl Pow2 for $f {
            type Output=$f;
            fn pow2(self) -> $f {
                self.clone() * self
            }
        }
        $crate::impl_tryop_from_op!(Pow2, pow2, $f);

        impl ScalarMul<$f> for $f {
            fn scalar_mul(self, rhs:&$f) -> $f {
                self*rhs.clone()
            }
        }
    }
}

macro_rules! binary_int_impl_try {
    ($tr:ident, $fn:ident, $f:ident) => {
        paste!(
        impl [<Try $tr>] for $f {
            type Output=$f;
            type Error=[<$tr Error>];
            fn [<is_  $fn able_by>](&self, rhs:&$f) -> Result<(), [<$tr Error>]> {
                self.clone()
                    .[<try_ $fn>](rhs.clone())
                    .map(|_|())
            }

            fn [<try_ $fn>](self, rhs:$f) -> Result<$f, [<$tr Error>]> {
                match self.[<checked_ $fn>](rhs) {
                    Some(res) => Ok(res),
                    None      => Err(OverflowError.into())
                }
            }
        });
    };
}

macro_rules! binary_float_impl_try {
    ($tr:ident, $fn:ident, $f:ident) => {
        paste!(
        impl [<Try $tr>] for $f {
            type Output=$f;
            type Error=[<$tr Error>];
            fn [<is_ $fn able_by>](&self, rhs:&$f) -> Result<(),[<$tr Error>]> {
                self.clone()
                    .[<try_ $fn>](rhs.clone())
                    .map(|_|())
            }

            fn [<try_ $fn>](self, rhs:$f) -> Result<$f,[<$tr Error>]> {
                self.check_float_input()?;
                rhs.check_float_input()?;
                let res=<Self as std::ops::$tr>::$fn(self,rhs);
                res.check_output()?;
                Ok(res)
            }
        });
    };
}

macro_rules! impl_only_int {
    ($f:ident) => {
        binary_int_impl_try!(Add, add, $f);
        binary_int_impl_try!(Sub, sub, $f);
        binary_int_impl_try!(Mul, mul, $f);

        impl TryDiv for $f {
            type Output=$f;
            type Error =DivError;
            fn is_divable_by(&self, rhs:&$f) -> Result<(), DivError > {
                self.clone()
                    .try_div(rhs.clone())
                    .map(|_|())
            }

            fn try_div(self, rhs:$f) -> Result<$f, DivError> {
                DivisionByZeroError::try_new(&rhs)?;
                self.checked_div(rhs)
                    .ok_or(OverflowError.into())
            }
        }

        // impl TryDiv for $f {
        //     type Output=$f;

        //     fn 

        //     fn try_div(self,rhs:$f) -> Result<$f, DivError> {
        //         self.try_div(rhs)
        //     }
        // }
    }
}


macro_rules! impl_only_float {
    ($f:ident) => {

        binary_float_impl_try!(Add, add, $f);
        binary_float_impl_try!(Sub, sub, $f);
        binary_float_impl_try!(Mul, mul, $f);

        impl TryDiv for $f {
            type Output=$f;
            type Error=DivError;
            fn is_divable_by(&self, rhs:&$f) ->  Result<(),DivError> {
                self.clone()
                    .try_div(rhs.clone())
                    .map(|_|())
            }

            fn try_div(self, rhs:$f) -> Result<$f,DivError> {
                self.check_float_input()?;
                rhs.check_float_input()?;
                DivisionByZeroError::try_new(&rhs)?;
                let res=self / rhs;
                if res.is_finite() {
                    Ok(res)
                } else {
                    Err(OverflowError.into())
                }
            }
        }
        // impl TryDiv for $f {
        //     type Output=$f;
        //     fn try_div(self,rhs:$f) -> Result<$f,DivError> {
        //         self.try_div(rhs)
        //     }
        // }

        impl Max for $f {
            fn max<'a>(&'a self, rhs:&'a Self) -> &'a Self {
                if self > rhs {
                    self
                } else {
                    rhs
                }
            }
        }

        impl ConstPi for $f {
            const PI:$f=std::$f::consts::PI;
        }

        impl ConstRad2Deg for $f {
            const RAD2DEG:$f=180.0 / std::$f::consts::PI;
        }

        impl ConstDeg2Rad for $f {
            const DEG2RAD:$f=std::$f::consts::PI / 180.0;
        }

        $crate::impl_div2to10!(impl_divi4f, $f);

        impl NormSquared for $f {
            type Norm2T=$f;
            fn norm_squared(self) -> Nonnegative<Self::Norm2T> {
                Nonnegative::try_new(self.pow2()).unwrap()
            }
        }

        impl TryNormalize for $f {}

        impl $crate::operators::basic::Inv for $f {
            type Output=<Self as num_traits::Inv>::Output;
            fn inv(self) -> Self::Output {
                <Self as num_traits::Inv>::inv(self)
            }
        }

        impl TryPow<$f> for $f {
            type Output = $f;
            type Error  = PowError;
            fn is_powable_by(&self, rhs:&$f) -> Result<(),PowError> {
                CheckPowInput::check_pow_input(self,rhs)
            }

            fn try_pow(self, rhs:$f) -> Result<Self::Output,PowError> {
                self.is_powable_by(&rhs)
                    .map(|_|self.powf(rhs))
            }
        }

        impl TryPow<i16> for $f {
            type Output = $f;
            type Error = PowError;
            fn is_powable_by(&self, rhs:&i16) -> Result<(), PowError> {
                <Self as TryPow<$f>>::is_powable_by(&self,&rhs.clone().into())
            }
            fn try_pow(self, rhs:i16) -> Result<Self::Output, PowError> {
                <Self as TryPow<$f>>::try_pow(self, rhs.into())
            }
        }

        impl TryInv for $f {
            type Output=$f;
            type Error=InvError;
            fn is_invertible(&self) -> Result<(), InvError> {
                DivisionByZeroError::try_new(self)?;
                self.check_float_input()?;
                Ok(())
            }
            fn try_inv(self) -> Result<Self::Output,InvError> {
                self.is_invertible()
                    .map(|_|1.0/self)
            }
        }

        impl std::ops::Div<NonZero<$f>> for $f {
            type Output=$f;
            fn div(self, rhs:NonZero<$f>) -> $f {
                self / rhs.into_inner()
            }
        }

        impl IntegralDomain for $f {}

        impl TrySqrt for $f {
            type Output = Nonnegative<$f>;
            type Error  = SqrtError;
            fn is_sqrtable(&self) -> Result<(), SqrtError> {
                self.check_float_input()?;
                if self >= &0.0 {
                    Ok(())
                } else {
                    Err(SqrtError::SqrtOfNegativeNumberNotPossible)
                }           
            }
            fn try_sqrt(self) -> Result<Self::Output, SqrtError> {
                self.is_sqrtable()
                    .map(|_|Nonnegative::try_new($f::sqrt(self)).unwrap())
            }
        }
        

        impl_un_op_f!(Exp,      exp,      |s| $f::exp(s), $f);
        impl TryIntoReal for $f {
            type Output=$f;
            fn try_into_real(self) -> Option<$f> {
                Some(self)
            }
        }

        impl TryLog for $f {
            type Output = $f;
            type Error  = LogError;
            fn is_logable(&self) -> Result<(),LogError> {
                self.check_float_input()?;
                if self > &0.0 {
                    Ok(())
                } else {
                    Err(LogError::LogOfNonPositiveRealNumberNotPossible)
                }  
            }

            fn try_log(self)  -> Result<Self::Output,LogError> {
                self.is_logable()
                    .map(|_|self.ln())
            }
        }

        impl ScalarDiv<$f> for $f {
            fn scalar_div(self, rhs:&$f) -> $f {
                self / rhs.clone()
            }
        }

        impl TryScalarDiv<$f> for $f {
            type Error=DivError;
            fn try_scalar_div(self, rhs:&$f) -> Result<$f,DivError> {
                DivisionByZeroError::try_new(rhs)?;
                Ok(self / rhs.clone())
            }
        }

        impl FiniteDimensionalVectorspace<$f, 1> for $f {}

        impl Norm for $f {
            type NormT=$f;

            fn norm(self) -> Nonnegative<Self::NormT> {
                Nonnegative::try_new(self.abs()).unwrap()
            }
        }

        impl TryDistance for $f  {
            type TryDistT=$f;
            type Error=SubError;
            fn try_distance(self, rhs:impl Into<$f>) -> Result<Nonnegative<$f>, SubError> {
                let rhs:$f=rhs.into();
                Ok((rhs-self).norm())
            }
        }

        impl Distance for $f {
            type DistT=$f;
            fn distance(self, rhs:impl Into<$f>) -> Nonnegative<$f> {
                let rhs:$f=rhs.into();
                (rhs-self).norm()
            }
        }

        impl TryScalarproduct for $f {
            type TryScProdT = $f;
            fn try_scalar_product(self, rhs: Self) -> Option<$f> {
                Some(self.scalar_product(rhs))
            }
        }

        impl Scalarproduct for $f {
            type ScProdT=$f;
            fn scalar_product(self,rhs:Self) -> Self::ScProdT {
                self*rhs
            }
        }

        impl Basis<$f> for $f {
            fn basis() -> impl ExactSizeIterator<Item=$f> {
                std::iter::once($f::NONZERO)
            }
        }
        // $crate::parameters::parameters1::impl_parameters1_self!($f);

        impl Sinc for $f {
            type Output=$f;
            fn denominator(self) -> $f {
                self
            }
        }
        
        impl $crate::TrigonometricFunctions for $f {
            type Output=$f;
            fn sin(self) -> $f {
                $f::sin(self)
            }

            fn cos(self) -> $f {
                $f::cos(self)
            }

            fn tan(self) -> Result<$f,DivError> {
                let res=$f::tan(self);
                if res.is_finite() {
                    Ok(res)
                } else {
                    Err(DivisionByZeroError.into())
                }
            }
        }

        impl TryATan2 for $f {
            type Output=$f;
            fn try_atan2(sin:$f, cos:$f) -> Option<$f> {
                (sin != 0.0 || cos != 0.0).then(||
                    $f::atan2(sin, cos))
            }
        }

        impl CastFromf64 for $f {
            fn from_f64(value:f64) -> Self {
                value as $f
            }
        }

        impl Scalar for $f {
            fn basis_over_r() -> Vec<Self> {
                vec![1.0]
            }

            type RealType = $f;
        }
        
        impl RealNumber for $f {}

    }
}


macro_rules! impl_int {
    ($f:ident) => {
        impl_only_int!($f);
        impl_int_and_float!($f);
    }
}


macro_rules! impl_float {

    ($f:ident) => {
        impl_int_and_float!($f);
        impl_only_float!($f);
    }
}

macro_rules! impl_threshold {
    ($e:expr, $f:ident) => {
        impl Tolerance for $f {
            const THRESHOLD:$f=$e;
        }
    };
}

impl_int!(i32);

impl_float!(f64);
impl_threshold!(1e-10, f64);

impl_float!(f32);
impl_threshold!(1e-5, f32);


#[test]
fn test_atan2() {
    // should be true in the range [-pi, pi]
    for a in [-3.102, -1.23, 0.0, 1.134, 3.104] {
        let s=a.sin();
        let c=a.cos();
        let are=f64::atan2(s, c);
        assert!((are-a).is_small())
    }
}