use matrix::MatrixDyn;
use container_traits::{AnyParameters, IntoParameters, LinearContainerConstructError as LCCE};

use crate::ProblemBuilder;

use super::{OptimizationError, OptimizationOptions};
use algebra_traits::Scalar;


pub fn fsolve<
    F : Scalar,
    X : Clone+AnyParameters<F,LCCE>,
    Y : Clone+AnyParameters<F,LCCE>>(
        f    : impl Fn(X) -> Y,
        x0   : X,
        opts : Option<OptimizationOptions<F>>) -> Result<X, OptimizationError<F, X>> {
        ProblemBuilder::new(&f,x0)
           .set_target_to_zero()
           .options(opts.unwrap_or_default())
           .build().unwrap()
           .solve()
}

pub fn fsolve_with_der<
    F : Scalar,
    X : Clone+AnyParameters<F,LCCE>,
    Y : Clone+AnyParameters<F,LCCE>>(
        f    : impl Fn(X) -> Y,
        fder : impl Fn(X) -> MatrixDyn<F>,
        x0   : X,
        opts : Option<OptimizationOptions<F>>) -> Result<X, OptimizationError<F, X>> {
        ProblemBuilder::new(&f,x0)
            .set_target_to_zero()
            .options(opts.unwrap_or_default())
            .build().unwrap()
            .solve_with_der(&fder)
}

pub fn solve_inverse_problem<
    F : Scalar,
    X : Clone+AnyParameters<F,LCCE>,
    Y : Clone+IntoParameters<F>>(
        f    : impl Fn(X) -> Y,
        x0   : X,
        y    : Y,
        opts : Option<OptimizationOptions<F>>) -> Result<X, OptimizationError<F, X>> {
        ProblemBuilder::new(&f,x0)
            .target(y)
            .options(opts.unwrap_or_default())
            .build().unwrap()
            .solve()
}

pub fn solve_inverse_problem_with_der<
    F : Scalar,
    X : Clone+AnyParameters<F,LCCE>,
    Y : Clone+IntoParameters<F>>(
        f    : impl Fn(X) -> Y,
        fder : impl Fn(X) -> MatrixDyn<F>,
        x0   : X,
        y    : Y,
        opts : Option<OptimizationOptions<F>>) -> Result<X, OptimizationError<F,X>> {
        ProblemBuilder::new(&f,x0)
            .target(y)
            .options(opts.unwrap_or_default())
            .build().unwrap()
            .solve_with_der(&fder)
}

#[test]
fn test_scalar_optimization_problem() {
    let f = |x: f64| x * x;
    let x0 = 2.0;
    let y = 2.0;
    let xsol = solve_inverse_problem(f, x0, y, None).unwrap();
    assert!((xsol - (2.0 as f64).sqrt()).abs() < 1e-10);
}

#[test]
fn test_multidim_optimization_problem() {
    use algebra_traits::Norm;
    use algebra::{Vector2, Vector3};
    let f = |x: Vector2<f64>| Vector3::from([x[0] - 42.0, x[1] - 45.0, x[0] - x[1]]);
    let x0 = Vector2::from([40.0, 50.0]);
    let xsol = fsolve(f, x0, None).unwrap();
    assert!((f(xsol) - Vector3::from([1.0, -1.0, -1.0])).norm() < 1e-10);
}

#[test]
fn test_multidim_inverse_problem() {
    use algebra_traits::Norm;
    use algebra::{Vector2, Vector3};
    let f = |x: Vector2<f64>| Vector3::from([x[0], x[1], x[0] - x[1]]);
    let x0 = Vector2::from([40.0, 50.0]);
    let y=Vector3::from([42.0,45.0,0.0]);
    let xsol = solve_inverse_problem(f, x0, y.clone(),None).unwrap();
    assert!((f(xsol) - (y+Vector3::from([1.0, -1.0, -1.0]))).norm() < 1e-10);
}
