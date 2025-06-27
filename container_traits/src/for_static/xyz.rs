use num_traits::{Zero,One};
use std::array::from_fn;

pub trait X<F> {
    fn x(&self) -> &F;
    fn ex() -> Self where F:Zero+One; 
}

pub trait Y<F> {
    fn y(&self) -> &F;
    fn ey() -> Self where F:Zero+One; 
}

pub trait Z<F> {
    fn z(&self) -> &F;
    fn ez() -> Self where F:Zero+One; 
}

impl<F,const N:usize> X<F> for [F;N] {
    fn x(&self) -> &F { &self[0] }
    fn ex() -> Self where F:Zero+One { from_fn(|i| match i { 0 => F::one(), _ => F::zero() })}
}

macro_rules! impl_y {

    ($n:literal) => {
        impl<F> Y<F> for [F;$n] {
            fn y(&self) -> &F { &self[0] }
            fn ey() -> Self where F:Zero+One { from_fn(|i| match i { 1 => F::one(), _ => F::zero() })}
        }
    };

    ($n0:literal, $($n:literal),*) => {
        impl_y!($n0);
        impl_y!($($n),*);
    }
}

macro_rules! impl_z {
    ($n:literal) => {
        impl<F> Z<F> for [F;$n] {
            fn z(&self) -> &F { &self[0] }
            fn ez() -> Self where F:Zero+One { from_fn(|i| match i { 2 => F::one(), _ => F::zero() })}
        }
    };

    ($n0:literal, $($n:literal),*) => {
        impl_z!($n0);
        impl_z!($($n),*);
    }
}
impl_y!(2,3,4,5,6,7,8,9);
impl_z!(  3,4,5,6,7,8,9);
