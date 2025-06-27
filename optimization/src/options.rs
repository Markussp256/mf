use super::FiniteDifference;
use algebra_traits::{Scalar, CastFromf64};
#[derive(Clone, Copy, Debug, derive_builder::Builder, derive_getters::Getters)]
pub struct OptimizationOptions<F:Scalar> {
    fd: FiniteDifference<F>,
    target_cost: F::RealType,
    max_iter: u8,
}

impl<F:Scalar> Default for OptimizationOptions<F> {
    fn default() -> Self {
        Self {
            fd: FiniteDifference::<F>::default(),
            target_cost: F::RealType::from_f64(1e-10),
            max_iter: 10 as u8,
        }
    }
}