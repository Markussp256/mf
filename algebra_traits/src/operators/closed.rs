// why not for any?

use std::ops::{Neg,Add,Sub,Mul,Div};
use num_traits::Inv;
use super::{TryAdd, TrySub, TryMul, TryDiv, TryInv, Pow2};

macro_rules! def_closed {
    ($uc:ident) => {
        paste::paste!(
        pub trait [<Closed $uc>] : Sized+$uc<Output=Self> {}

        impl<F:Sized+$uc<Output=F>> [<Closed $uc>] for F {}

        );
    };
}
def_closed!(Neg);
def_closed!(Inv);
def_closed!(Add);
def_closed!(Sub);
def_closed!(Mul);
def_closed!(Div);
def_closed!(Pow2);
def_closed!(TryInv);
def_closed!(TryAdd);
def_closed!(TrySub);
def_closed!(TryMul);
def_closed!(TryDiv);