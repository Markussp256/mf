use algebra_traits::Scalar;

use container_traits::{AnyParameters, IntoParameters, LinearContainerConstructError as LCCE};
use super::{OptimizationError, OptimizationOptions, ProblemBuilder};


// solves s.t. solution gets not to far away from initial guess
pub fn solve_inverse_problem_regularized<
    F : Scalar,
    X : Clone+AnyParameters<F,LCCE>,
    Y : Clone+IntoParameters<F>>(
        f    : impl Fn(X) -> Y,
        x0   : X, // first guess
        y    : Y,
        rc   : F::RealType, // reg constant
        opts : Option<OptimizationOptions<F>>) -> Result<X, OptimizationError<F, X>> {
        ProblemBuilder::new(&f,x0)
           .target(y)
           .options(opts.unwrap_or_default())
           .build().unwrap()
           .regularize(rc)
           .solve()
}

pub fn fsolve_regularized<
    F : Scalar,
    X : Clone+AnyParameters<F,LCCE>,
    Y : Clone+AnyParameters<F,LCCE>>(
        f    : impl Fn(X) -> Y,
        x0   : X,
        rc   : F::RealType,
        opts : Option<OptimizationOptions<F>>) -> Result<X, OptimizationError<F, X>> {
    ProblemBuilder::new(&f,x0)
           .set_target_to_zero()
           .options(opts.unwrap_or_default())
           .build().unwrap()
           .regularize(rc)
           .solve()
}

#[test]
fn test_inv_problem_reg() {
    use algebra_traits::Tolerance;
    let f = |x: [f64;2]| x[1] - x[0];
    let res = solve_inverse_problem_regularized(&f, [2.0, 5.0], 0.0, 1.0, None).unwrap();
    let expected=[3.0, 4.0];
    for i in 0..2 {
        assert!(res[i].is_close_to(expected[i])); //  distance([3.0, 4.0]) < 1e-6
    }
}