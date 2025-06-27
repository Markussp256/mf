// we can not implement display for (usize,usize) so we use newtype pattern

use std::fmt::Display;
type U2=(usize,usize);

#[derive(Clone, Copy, Debug, PartialEq, derive_more::From, derive_more::Into)]
pub struct MatrixDimensions(U2);

impl Display for MatrixDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} x {}", self.0.0, self.0.1)   
    }
}