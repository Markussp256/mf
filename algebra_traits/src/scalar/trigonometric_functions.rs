use crate::{ConstNonZero, DivError, Field, Norm, Scalar, TryDiv, TryPow};

pub trait TrigonometricFunctions {
    type Output;
    fn sin(self) -> Self::Output;
    // we could derive cos, tan from sin but we leave that to the std library
    fn cos(self) -> Self::Output;
    fn tan(self) -> Result<Self::Output,DivError>;
}

pub trait TryATan2 : Sized {
    type Output;
    // required method
    fn try_atan2(sin:Self, cos:Self) -> Option<Self::Output>;

    // provided method
    fn try_atan2_generic<T:ConstNonZero+TryDiv<Output=Self>>(sin:T, cos:T) -> Option<Self::Output> {
       Self::try_atan2(sin.div_nz(), cos.div_nz())
    }
}


#[macro_export]
macro_rules! impl_atan2 {
    ($f:ident) => {
        impl Atan2 for $f {
            type Output=$f;
            fn atan2(cos:$f, sin:$f) -> Option<$f> {
                (!cos.is_zero() || !sin.is_zero()).then(||
                    $f::atan2(cos, sin))
                }
        }
    }
}



// sinc(x)=sin(x)/x has a removable singularity
// close to the singularity we use the Taylor expansion up to degree 4/5
// and evaluate it using Horner scheme.
// further away from the singularity we use the definition
// we test that the two formulas yields the same up to the expected precision

pub fn sinc_taylor<T:Field+Clone>(z: T) -> T {
    let z2 =|| z.clone().pow2();
    let one=||<T as num_traits::One>::one();
         one()-z2().div2().div3()
       *(one()-z2().div4().div5())
       *(one()-z2().div6().div7())
       *(one()-z2().div8().div9())
}


#[test]
fn test_sinc_taylor() {
    for z in vec![0.99e-3 as f64, 1e-4, 1e-6, 1e-8, -0.99e-3, -1e-6, -1e-8] {
        assert!((sinc_taylor(z) - z.sin() / z).abs() < 1e-14);
    }
}



// note that for sinc we might actually omit the taylor expansion and deal only with the case z == 0 seperately
// however the Taylor exp might be more efficient than calling sin
pub trait Sinc : Sized {
    type Output : Clone + Scalar + TryDiv<Output=Self::Output>; // Scalar actually also includes TryDiv but for some reason downstream crates dont accept this 
    // required methods
    fn denominator(self) -> Self::Output;

    // provided methods
    fn sinc(self) -> Self::Output {
        Self::sinc_from_den(self.denominator())
    }

    fn sinc_from_den(den:Self::Output) -> Self::Output {
        let threshold=<Self::Output as Scalar>::RealType::from(10).try_pow(-3).ok().unwrap();
        if den.clone().norm() < threshold {
            sinc_taylor(den)
        } else {
            <Self::Output as TryDiv>::try_div(den.clone().sin(), den).ok().unwrap()
        }
    }
}





// impl_trigonometry!(f64, f64);
