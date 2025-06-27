pub mod error;
pub use error::OptimizationError;

pub mod finite_difference;
pub use finite_difference::{FiniteDifference, FiniteDifferenceMethod};

pub mod jacobian;
pub use jacobian::{jacobian, jacobian_dvec}; // , uncertainties

pub mod least_squares;
pub use least_squares::try_solve_least_squares;

pub mod fixpoint_iteration;
pub use fixpoint_iteration::fix_point_iteration;

pub mod fsolve;
pub use fsolve::{fsolve, solve_inverse_problem};

pub mod options;
pub use options::{OptimizationOptions, OptimizationOptionsBuilder};

pub mod fsolve_regularized;
pub use fsolve_regularized::{fsolve_regularized, solve_inverse_problem_regularized};

pub mod problem;
pub use problem::{Problem, ProblemBuilder, ProblemBuilderError};

use algebra::VectorDyn;
use container_traits::{AnyFromParameters, IntoParameters, LinearContainerConstructError};

fn into_dvec<F,C:IntoParameters<F>>(c: C) -> VectorDyn<F> {
    VectorDyn::from_iter(c.into_parameters())
}

fn from_dvec<F, C:AnyFromParameters<F,LinearContainerConstructError>>(dvec: VectorDyn<F>) -> C {
    let iter=dvec.into_parameters();
    C::any_from_iter(None, iter).ok().unwrap()
}
