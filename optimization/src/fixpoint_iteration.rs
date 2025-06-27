// assuming we have a function we try to find the fixpoint and its a contraction

use super::OptimizationError;
use algebra_traits::{Nonnegative, Distance, TryDistance};


// fixed number of iterations
pub fn fix_point_iteration<X>(
    xstart:X,
    f:impl Fn(X) -> X,
    niter:Option<u8>) -> X {
    let mut x=xstart;
    let niter=niter.unwrap_or(10);
    for _ in 0..niter {
        x=f(x);
    }
    x
}


// with abbruchkriterium
pub fn fix_point_iteration_with_termination_condition<X:Clone>(
    xstart:X,
    f:impl Fn(X) -> X,
    fquite:impl Fn(X,X) -> Result<bool, OptimizationError<(), X>>,
    maxiter:Option<u8>) -> Result<X,    OptimizationError<(), X>> {
    let maxiter=maxiter.unwrap_or(10);
    let mut x=xstart;
    let mut iter=0;
    while iter < maxiter {
        let xold=x.clone();
        x=f(x);
        // fn is_some_and would move tol
        if fquite(xold, x.clone())? { break; }
        iter+=1;
    }
    if iter == maxiter {
        return Err(OptimizationError::MaximalIteration(iter));
    }
    Ok(x)
}


pub fn fix_point_iteration_try_distance<X:Clone+TryDistance<DistT=D>, D : Clone+PartialOrd>(
    xstart:X,
    f:impl Fn(X) -> X,
    tol:Nonnegative<D>,
    maxiter:Option<u8>) -> Result<X, OptimizationError<(), X>> {
    fix_point_iteration_with_termination_condition(
        xstart,
        f,
        |xold, x| match xold.clone().try_distance(x.clone()) {
            Ok(d) => Ok(d < tol.clone().into_signed()),
            Err(_) => Err(OptimizationError::DifferentOutputDimensions(xold,x) )},
        maxiter)
}


pub fn fix_point_iteration_distance<X:Clone+Distance<DistT=D>, D : Clone+PartialOrd>(
    xstart:X,
    f:impl Fn(X) -> X,
    tol:Nonnegative<D>,
    maxiter:Option<u8>) -> Result<X, OptimizationError<(), X>> {
        fix_point_iteration_with_termination_condition(
            xstart,
            f,
            |xold, x|
                Ok(xold.distance(x) < tol.clone().into_signed()),
            maxiter)
}

