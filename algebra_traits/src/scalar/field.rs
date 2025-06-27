// Note that a Field is not a multiplicativegroup because it contains a zero.


use crate::*;
use num_traits::One;
use std::ops::Div;

pub trait Ring : AdditiveGroup
                +ClosedMul
                +ClosedPow2 {}

impl<T : AdditiveGroup
        +ClosedMul
        +ClosedPow2> Ring for T {}

pub trait Field: Ring
                +IntegralDomain
                +DivBySmallNatural
                +ClosedTryInv<Error=InvError>
                +ClosedTryDiv<Error=DivError>
                +One
                +Div<NonZero<Self>,Output=Self> {}

impl<T : Ring
        +IntegralDomain
        +DivBySmallNatural
        +ClosedTryInv<Error=InvError>
        +ClosedTryDiv<Error=DivError>
        +One
        +Div<NonZero<T>,Output=T>> Field for T {}