
use std::{num::NonZeroU32, ops::Add};
use std::ops::{Mul, Neg};
use num_traits::{Zero, One, Inv};

// group power can be used as long as multiples resp. powers can be added resp. multiplied without failure.
// therefore we use any.  

// need to add this macro at the start of the file such that it can be included without exporting
macro_rules! group_power {
    ($tr_name:ident::$op_name:ident,
        $base_fn:ident
        $(++ $id_tr:ident::$id_fn:ident )?
        $(+ $inv_tr:ident:: $inv_fn:ident)? , $ity:ident $(as $ity2:ident)?) => {
        pub trait $tr_name : Clone $(+$id_tr)? $(+ $inv_tr<Output=Self> )? {

            // required method
            fn $base_fn(self, rhs:Self) -> Self;

            // provided method
            fn $op_name(&self, i:$ity) -> Self {
                $(let i:$ity2=i.into();)?
                match i {
                  $(-1 => self.clone().$inv_fn(), )?
                  $( 0 => Self::$id_fn(),)?
                     1 => self.clone(),
                     2 => self.clone().$base_fn(self.clone()),
                     i => {
                         let i2 = i / 2;
                         $(let i2:$ity=<$ity2 as TryInto<$ity>>::try_into(i2).unwrap();)?
                         let selfi2=self.$op_name(i2); // recursive calls 
                         let selfi22 = selfi2.clone().$base_fn(selfi2);
                          match i % 2 {
                         $(-1 =>  selfi22.$base_fn(self.clone().$inv_fn()), )?
                            0 =>  selfi22,
                            1 =>  selfi22.$base_fn(self.clone()),
                            _ =>  { panic!("remainder after division by 2 should be -1, 0 or 1"); }
                         }
                     }
                 }
            }
        }
    };
}
group_power!(MulNZU::mulnzu, base_add   ,  NonZeroU32 as u32);
group_power!(MulU::mulu,     base_add ++Zero::zero ,            u32);
group_power!(MulI::muli,     base_add  ++Zero::zero + Neg::neg , i32);

impl<T:Clone+Add<Output=T>+Zero+Neg<Output=T>> MulI for T {
    fn base_add(self,rhs:Self) -> Self {
        self+rhs
    }
}

group_power!(PowNZU::pownzu, base_mul,     NonZeroU32 as u32);
group_power!(PowU::powu,     base_mul ++One::one   ,            u32);
group_power!(PowI::powi,     base_mul ++One::one   + Inv::inv , i32);

impl<T:Clone+Mul<Output=T>+One+Inv<Output=T>> PowI for T {
    fn base_mul(self,rhs:Self) -> Self {
        self*rhs
    }
}


#[test]
fn test_mul_i32() {
    let val = 1.1;
    for n in -10..10 {
        assert_eq!(val * (n as f64), val.muli(n));
    }
}

#[test]
fn test_pow_i32() {
    let val: f64 = 1.1;
    for n in -10..10 {
        let lhs = num_traits::Float::powi(val, n);
        let rhs: f64 = val.powi(n);
        assert!((lhs - rhs).abs() < 1e-6);
    }
}