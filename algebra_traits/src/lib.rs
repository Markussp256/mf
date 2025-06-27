// in some macros we call $crate::$tr
// so we need some basic traits in the
// crate root
pub use std::ops::{Neg,Add,Sub,Div,Mul};
pub use num_traits::{Inv,Pow};

pub mod group;
pub use group::*;

pub mod operators;
pub use operators::*;

pub mod linear_algebra;
pub use linear_algebra::*;

// pub mod parameters;
// pub use parameters::{Parameters, Parameters1, NumberOfDegreesOfFreedom};

pub mod metric;
pub use metric::*;

#[cfg(feature = "nalgebra_support")]
mod nalgebra_impl;

pub mod partial_ord;
pub use partial_ord::*;

pub mod scalar;
pub use scalar::*;
// parameters trifold end

pub mod set_theory;
pub use set_theory::{Complement, ConstElement};