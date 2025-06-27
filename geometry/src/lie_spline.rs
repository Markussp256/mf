use std::{marker::PhantomData, ops::Mul};

use algebra::spline::{Knot, Spline};
use super::tangent_bundle::weighted_average;
use algebra_traits::{Exp, TryLog, Vectorspace};

use container_traits::Parameter;
use geometry_traits::LieGroup;
use splines::Interpolation;

#[derive(Debug, Clone)]
pub struct LieSpline<T, M, E>(Vec<(Spline<T, f64>, M)>,PhantomData<E>);

impl<T: Parameter<f64>, M, E> LieSpline<T, M, E> {
    pub fn new(vs: Vec<(Spline<T, f64>, M)>) -> Self {
        Self(vs,PhantomData::<E>)
    }

    pub fn from_lagrange_splines_and_ctrl_vals(lsps:Vec<Spline<T,f64>>, ms:Vec<M>) -> Option<Self> {
        (lsps.len() == ms.len()).then(||
            Self::new(lsps.into_iter()
                             .zip(ms.into_iter())
                             .collect())
        )
    }

    pub fn lagrange_splines(&self) -> Vec<&Spline<T, f64>> {
        self.0
            .iter()
            .map(|(sp,_)|sp)
            .collect()
    }

    pub fn control_values(&self) -> Vec<&M> {
        self.0
            .iter()
            .map(|(_,m)|m)
            .collect()
    }
}

impl<T    : Clone + PartialOrd + Parameter<f64>,
     M    : Clone + LieGroup<f64,E> + PartialEq + TryLog<Output=LogM>,
     E,
     LogM : Clone+Mul<f64,Output=LogM>+Exp<Output=M>+Vectorspace<f64>> LieSpline<T, M, E> {
    pub fn eval(&self, t: T, init: Option<M>) -> M {
        weighted_average(
            self.0
                      .clone()
                      .into_iter()
                      .map(|(lsp, m)| (lsp.eval(t.clone()), m))
                      .collect(),
            init,
        )
    }

    pub fn from_vec(io_pairs: Vec<(T, M)>, ipt: Interpolation<f64, f64>) -> Self {
        let (control_nodes, control_values): (Vec<_>, Vec<_>) = io_pairs.into_iter().unzip();
        let n = control_nodes.len();
        let lagrange_splines = (0..n)
            .into_iter()
            .map(|i| {
                Spline::try_new(
                    (0..n)
                        .into_iter()
                        .map(|j| Knot::new(control_nodes[j].clone(), if i == j { 1.0 } else { 0.0 }))
                        .collect(),
                    ipt,
                )
                .unwrap()
            })
            .collect();
        Self::from_lagrange_splines_and_ctrl_vals(lagrange_splines, control_values).unwrap()
    }
}
