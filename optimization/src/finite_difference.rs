use algebra_traits::{NonZero, Scalar, TryDiv, TrySub, CastFromf64};

#[derive(Clone, Copy, Debug)]
pub enum FiniteDifferenceMethod {
    Forward,
    Backward,
    Centered,
}

#[derive(Clone, Copy, Debug)]
pub struct FiniteDifference<F> {
    step: NonZero<F>,
    fdm: FiniteDifferenceMethod,
}

impl<F> FiniteDifference<F> {
    pub fn new(step: NonZero<F>, fdm: FiniteDifferenceMethod) -> Self {
        Self { step, fdm }
    }

    pub fn apply<
        Y: Clone+TrySub<Output = Y> + TryDiv<F>>(
        &self,
        f: impl Fn(F) -> Y,
        f0: &Y,
    ) -> <Y as TryDiv<F>>::Output where F:Scalar {
        let h = self.step.clone().into_inner();
        let hc=||h.clone();

        let subdiv=|u:Y,v,w|u.try_sub(v).unwrap()
                                                                         .try_div(w).unwrap();
        match self.fdm {
            FiniteDifferenceMethod::Forward =>  subdiv(f(hc()), f0.clone(), hc()),
            FiniteDifferenceMethod::Backward => subdiv(f0.clone(), f(-hc()), hc()),
            FiniteDifferenceMethod::Centered => subdiv(f(hc()), f(-hc()), hc()+hc()),
        }
    }
}

impl<F:Scalar> Default for FiniteDifference<F> {
    fn default() -> Self {
        Self {
            step: NonZero::try_new(F::from(F::RealType::from_f64(1e-8))).unwrap(),
            fdm: FiniteDifferenceMethod::Centered,
        }
    }
}
