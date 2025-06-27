use std::ops::Mul;

use algebra_traits::{Exp, TryLog, Scalar, Vectorspace};
use super::Manifold;

pub trait LogableManifold<F:Scalar,E> : Manifold<F,E> + TryLog
where   <Self as TryLog>::Output : Exp<Output =Self> + Vectorspace<F>,
F : Mul<<Self as TryLog>::Output,
Output= <Self as TryLog>::Output> { }