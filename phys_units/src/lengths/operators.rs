use super::generic::*;
use crate::duration::Duration_generic::Duration;


crate::physical_quantity::impl_mul_div_vars!(Duration, Jerk, Acceleration);
crate::physical_quantity::impl_mul_div_vars!(Duration, Acceleration, Speed);
crate::physical_quantity::impl_mul_div_vars!(Duration, Speed, Length);

crate::physical_quantity::impl_pow2_sqrt!(Length, Area);
