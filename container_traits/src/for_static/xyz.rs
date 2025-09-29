use num_traits::{Zero,One};
use std::array::from_fn;


macro_rules! def {
    ($uc:ident, $lc:ident, $euc:ident, $elc:ident, $j:literal $(, $i:literal)*) => {

        pub trait $uc<F> {
            fn $lc(&self) -> &F;
        }

        pub trait $euc<F> {
            fn $elc() -> Self where F:Zero+One; 
        }

        $(
            impl<F> $uc<F> for [F;$i] {
                fn $lc(&self) -> &F { &self[$j] }
            }

            impl<F> $euc<F> for [F;$i] {
                fn $elc() -> Self where F:Zero+One { from_fn(|i| match i { $j => F::one(), _ => F::zero() })}
            }
        )*
    };
}
def!(X,x,EX,ex,0,1,2,3,4,5,6,7,8,9);
def!(Y,y,EY,ey,1,  2,3,4,5,6,7,8,9);
def!(Z,z,EZ,ez,2,    3,4,5,6,7,8,9);
