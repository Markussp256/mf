
// if possible
// proc macros ScalarMul, ScalarTryDiv should be used instead

#[macro_export]
macro_rules! scalar_mul_generic {
    ($name:ident<$t:ident $(, $n:ident )*>) => {
        
        impl<T:std::ops::Mul<T2, Output=TR>, T2:Clone, TR  $(, const $n:usize)*> std::ops::Mul<T2> for $name<T  $(, $n)*> {
            type Output=$name<TR  $(, $n)*>;
            fn mul(self, rhs:T2) -> Self::Output {
                self.map(|aij|aij * rhs.clone())
            }
        }
    
        impl<T:algebra_traits::TryDiv<T2, Output=TR>, T2:num_traits::Zero+Clone, TR  $(, const $n:usize)*> algebra_traits::TryDiv<T2> for $name<T $(, $n)*> {
            type Output=$name<TR  $(, $n)*>;
            fn try_div(self, rhs:T2) -> Option<Self::Output> {
                if rhs.is_zero() {
                    None
                } else {
                    Some(self.map(|aij|aij.try_div(rhs.clone()).unwrap()))
                }
            }
        }

        // can not implement generically for T*f 
        impl<T2, TR  $(, const $n:usize)*> std::ops::Mul< $name<T2  $(, $n)*>> for f64 where f64 :std::ops::Mul<T2,Output=TR> {
            type Output=$name<TR  $(, $n)*>;
            fn mul(self, rhs:$name<T2  $(, $n)*>) -> Self::Output {
                rhs.map(|rij|self.clone() * rij)
            }
        }
    }
}

#[macro_export]
macro_rules! scalar_mul {
    ($name:ident <$t:ident $(, $n:ident)* > ) => {

        impl<T:algebra_traits::Scalar $(, const $n:usize )* > std::ops::Mul<T> for $name<T  $(, $n)*> {
            type Output=Self;
            fn mul(self, rhs:T) -> Self {
                self.map(|aij|aij * rhs.clone())
            }
        }
    
        impl<T:algebra_traits::Scalar $(, const $n:usize )*> algebra_traits::TryDiv<T> for $name<T $(, $n)*> {
            type Output=Self;
            fn try_div(self, rhs:T) -> Option<Self> {
                (!rhs.is_zero()).then(||
                self.map(|aij|aij.try_div(rhs.clone()).unwrap()))
            }
        }

        impl<T:algebra_traits::Scalar+From<f64> $(, const $n:usize )*> std::ops::Mul<$name<T $(, $n)*>> for f64 {
            type Output=$name<T $(, $n)*>;
            fn mul(self, rhs:$name<T $(, $n)*>) -> Self::Output {
                rhs.map(|rij|T::from(self) * rij)
            }
        }
    }
}