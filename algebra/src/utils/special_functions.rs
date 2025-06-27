use algebra_traits::{ComplexNumber, MulI, Scalar, TryLog, TryPow};

use num_traits::{Zero,One};


// here we implement functions with a removable singularity
// close to the singularity we use the Taylor expansion up to degree 4/5
// and evaluate it using Horner scheme.
// further away from the singularity we use the definition
// we test that the two formulas yields the same up to the expected precision

#[cfg(test)]
use crate::c64;

#[cfg(test)]
use algebra_traits::Exp;

fn fromi32<T:One+MulI>(i:i32) -> T {
    T::one().muli(i)
}

fn pow10<T:Scalar>(i:i16) -> T::RealType {
    T::RealType::from(10 as i16).try_pow(i).unwrap()
} 

// fn zp1tm1dz_taylor<T:Scalar>(z: T, t: T::RealType) -> T {
//     // function z -> ((z+1)^t-1)/z
//     if (t.clone() + T::RealType::one()) * z.clone().norm().into_signed() > pow10::<T>(-3) {
//         panic!(
//             "this function is not expected to take such values"
//         );
//     }
//     let ts:T=T::from(t);
//     let mut val = T::one();
//     for i in (1..5).rev() {
//         let if64: T = fromi32(i);
//         val = T::one() + (ts.clone() - if64.clone()).try_div(if64 + T::one()).unwrap() * z.clone() * val;
//     }
//     ts * val
// }

// #[test]
// fn test_zp1tm1dz_taylor() {
//     use algebra_traits::{TryPow, TryDiv};
//     for t in vec![-999.9 as f64, -3.4, -0.1, 0.0, 0.1, 2.7, 42.0, 999.9] {
//         // for t with large absolute value we expect a decrease in accuracy
//         for zm1norm in vec![0.99e-3 as f64, 1e-4, 1e-6, 1e-8]
//             .iter()
//             .filter(|&zn| zn.abs() * (1.0+ t.abs()) < 1e-3)
//         {
//             for k in 0..8 {
//                 let zm1 =
//                     zm1norm.clone() * c64::exp(2.0* std::f64::consts::PI * (k as f64) / 8.0 * c64::i());
//                 let z = zm1 + c64::one();
//                 assert!(
//                     (zp1tm1dz_taylor(zm1, t) - (z.try_pow(t).unwrap() - c64::one()).try_div(zm1).unwrap()).norm().into_signed()
//                         <= 1e-14 / zm1norm * t.abs()
//                 );
//             }
//         }
//     }
// }

// pub fn ztm1dzm1<T:ComplexNumber>(z: T, t: T::RealType) -> T {
//     // function z -> (z^t-1)/(z-1)
//     if t.clone().norm() > pow10::<T>(-3) {
//         panic!("this function is intended for moderate values of t");
//     }
//     let zm1 = z.clone() - T::one();
//     if zm1.clone().norm() * (t.clone().norm()+Nonnegative::<T::RealType>::one()) < pow10::<T>(-3) {
//         zp1tm1dz_taylor(zm1, t)
//     } else {
//         (z.try_pow(t).unwrap() - T::one()).try_div(zm1).unwrap()
//     }
// }

pub fn lndzm1<T:Clone+ComplexNumber+TryLog<Output=T>>(z: T) -> T {
    // function z -> log(z)/(z-1) which is approximately 1-z/2 at 1
    if z.real() <= &T::RealType::zero() && z.imag() == &T::RealType::zero() {
        panic!("tried to compute logarithm of value on nonpositive real axis")
    }
    let zm1 = z.clone().sub(T::one());
    if zm1.clone().norm() < pow10::<T>(-3) {
        lnzp1dz_taylor(zm1)
    } else {
        z.try_log().unwrap()
         .try_div(zm1).unwrap()
    }
}

fn lnzp1dz_taylor<T:Scalar>(z: T) -> T {
    // function z -> ln(z+1)/z which has Taylor expansion 1-z/2 + z^2/3 - z^3/4
    let n = 5;
    let mut val = T::one().try_div(fromi32::<T>(n)).unwrap();
    for i in (1..n).rev() {
        val = T::one().try_div(fromi32::<T>(i)).unwrap().sub(z.clone().mul(val))
    }
    val
}

#[cfg(test)]
use algebra_traits::{Norm, TryDiv, RealAndImag};

#[test]
fn test_lnzp1dz_taylor() {
    for znorm in vec![0.99e-3 as f64, 1e-4, 1e-6, 1e-8].into_iter() {
        for k in 0..8 {
            let z = znorm * c64::exp(2.0 * std::f64::consts::PI * (k as f64) / 8.0 * c64::i());
            let zp1 = z.clone() + c64::one();
            assert!((lnzp1dz_taylor(z.clone()) - zp1.try_log().unwrap().try_div(z).unwrap()).norm() < 1e-15 / znorm);
        }
    }
}

fn expm1dz_taylor<T:Clone+Scalar>(z: T) -> T {
    let one=fromi32::<T>(1);
    let mut res=one.clone();
    for i in (2..5).rev() {
        res = res*z.clone().try_div(fromi32::<T>(i)).unwrap() + one.clone();
    }
    res
}

#[test]
fn test_expm1dz_taylor() {
    for znorm in vec![0.99e-3 as f64, 1e-4, 1e-6, 1e-8] {
        for k in 0..8 {
            let z = znorm * c64::exp(2.0 * std::f64::consts::PI * (k as f64) / 8.0 * c64::i());
            assert!((expm1dz_taylor(z.clone()) - (z.clone().exp() - c64::one()).try_div(z).unwrap()).norm() < 1e-15 / znorm);
        }
    }
}

// maybe use exp_m1
// https://doc.rust-lang.org/std/primitive.f32.html#method.exp_m1
pub fn expm1dz<T:Clone+Scalar>(z: T) -> T {
    // function z -> (exp(z)-1)/z which is approximately 1+z/2 at 1
    if z.clone().norm() < pow10::<T>(-3) {
        expm1dz_taylor(z)
    } else {
        (z.clone().exp() - T::one()).try_div(z.clone()).unwrap()
    }
}
