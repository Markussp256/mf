// for vec there is no unique zero
// so we can not implement num_traits::Zero,
// However we can test if it is a zero


// we name it is_the_zero or is_a_zero to emphasize that there is exacly one or multiple zeros
// this also distinguish it from num_traits::is_zero
// if we would name it simply is_zero, compiler always asks which one to take if both are implemented
pub trait IsAZero {
    fn is_a_zero(&self) -> bool;
}

macro_rules! impl_is_a_zero_from_iter {
    ($name:ident<$t:ident $(, $n:ident )*>) => {
        impl<$t:num_traits::Zero $(, const $n:usize)* > $crate::scalar::zero::IsAZero for $name<$t $(,$n)* > {
            fn is_a_zero(&self) -> bool {
                self.iter()
                    .all(<$t as num_traits::Zero>::is_zero)
            }
        }
    };
}
impl_is_a_zero_from_iter!(Vec<T>);



impl<T:IsAZero, const N:usize> IsAZero for [T;N] {
    fn is_a_zero(&self) -> bool {
        self.iter()
            .all(<T as IsAZero>::is_a_zero)
    }
}

