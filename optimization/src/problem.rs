
use num_traits::One;
use container_traits::{for_dynamic::FromElement, AnyFromParameters, AnyParameters, Concat, Concatenated, IntoParameters, LinearContainerConstructError as LCCE};

use algebra_traits::{Norm, Scalar, ScalarMul, TrySub, TryAdd};

use algebra::VectorDyn;

use matrix::MatrixDyn;
use matrix_traits::{Matrix, MatrixTryConstruct};
use super::{from_dvec, into_dvec, OptimizationError, OptimizationOptions};

#[derive(Clone, Debug, derive_builder::Builder)]
pub struct Problem<F:Scalar,
                   X,
                   Y,
                   Func : Fn(X) -> Y> {
    #[builder(setter(into))]
    function: Box<Func>,

    first_guess: X,

    target: Y,

    #[builder(setter(into))]
    weights: VectorDyn<F::RealType>,

    #[builder(default)]
    options:OptimizationOptions<F>
}

impl<F    : Scalar,
     X    : Clone,
     Y    : Clone+IntoParameters<F>,
     Func : Fn(X) -> Y> ProblemBuilder<F,X,Y,Func> {

        fn get_y_len(y:Y) -> usize {
            y.into_parameters().count()
        }

        pub fn new(f:Func, x:X) -> Self {
            Self{
                function:Some(Box::new(f)),
                first_guess: Some(x),
                target: None,
                weights: None,
                options:Some(OptimizationOptions::default())
            }
        }

        pub fn set_target_to_zero(& mut self) -> & mut Self where Y : AnyFromParameters<F,LCCE> {
            let f=self.function.as_ref().unwrap();
            let len=Self::get_y_len(f(self.first_guess.clone().unwrap()));
            self.target=Some(from_dvec(VectorDyn::from_element(len, F::zero())));
            self
        }

        pub fn set_weights_to_one(& mut self) -> & mut Self {
            let len=Self::get_y_len(self.target.clone().unwrap());
            self.weights=Some(VectorDyn::from_element(len, F::RealType::one()));
            self
        }
    }
// pub struct ProblemBuilder<X,Y> {
//     function: Option<Box<dyn Fn(&X) -> Y>>,
//     first_guess: Option<X>,
//     target:Option<Y>,
//     derivative: Option<Box<dyn Fn(&X) -> DMatrix<f64>>>,
//     options:Option<OptimizationOptions>
// }

// impl<X,Y> ProblemBuilder<X,Y> {
//     pub fn default() -> Self {
//         ProblemBuilder{
//             function:None,
//             first_guess:None,
//             target:None,
//             derivative:None,
//             options:None
//         }
//     }

//     pub fn function<F:'static+Fn(&X) -> Y>(& mut self, f:F) -> & mut Self {
//         self.function=Some(Box::new(f));
//         self
//     }

//     pub fn build(self) -> Problem<X,Y> {
//         Problem{
//             function:self.function.unwrap(),
//             first_guess:self.first_guess.unwrap(),
//             target:self.target.unwrap(),
//             derivative:self.derivative,
//             options:self.options
//         }
//     }

// }


// solving regularized problem if derivative of original problem is not provided is slow

impl<F    : Scalar,
     X    : Clone+IntoParameters<F>,
     Y    : Clone,
     Func : Fn(X) -> Y> Problem<F,X,Y,Func> {
    pub fn regularize(self, reg_const:F::RealType) -> Problem<F,X,
                                                      Concatenated<X,Y>,
                                                      impl Fn(X) -> Concatenated<X,Y>> {
        let first_guess=self.first_guess.clone();
        // let xdim=first_guess.parameters().len();
        // let xdim=X::NDOFS;
        let target=Concatenated::new(first_guess.clone(), self.target.clone());
        let ndofs=first_guess.clone().into_parameters().count();
        let weights=VectorDyn::from_element(ndofs, reg_const).concat(self.weights.into()).into();
        
        Problem{function:(move |x:X| Concatenated::new(x.clone(),(self.function)(x.clone()))).into(),
                first_guess,
                target,
                weights,
                options:self.options.clone()}
    }
}

impl<F    : Scalar,
     X    : AnyParameters<F,LCCE>+Clone,
     Y    : IntoParameters<F>+Clone,
     Func : Fn(X) -> Y> Problem<F,X,Y,Func> {
    fn numerical_derivative(&self, x:X) -> MatrixDyn<F>
    {
        super::jacobian(&self.function, x, self.options.fd().clone())
    }

    pub fn solve(&self) -> Result<X, OptimizationError<F,X, LCCE>> {
        self.solve_with_der(|x:X| self.numerical_derivative(x))
    }

    pub fn solve_with_der(&self, derivative:impl Fn(X) -> MatrixDyn<F>) -> Result<X, OptimizationError<F,X>> {
        let f=&self.function;
        let opts: OptimizationOptions<F> = self.options.clone();
        let mut x=self.first_guess.clone();
        let y_dvec:VectorDyn<F>=into_dvec(self.target.clone());
        let weights=||self.weights.clone();
        let mut iter:u8=0;
        while &iter < opts.max_iter() {
            let jac=derivative(x.clone());
            let wjac=MatrixDyn::try_from_rows(
                    jac.into_rows()
                       .zip(weights().into_iter())
                       .map(|(r,wi)|r.scalar_mul(&F::from(wi)))).unwrap();
            let res:VectorDyn<F>=y_dvec.clone().try_sub(into_dvec(f(x.clone()))).unwrap();
            let wres=container_traits::vec_op::try_binary_operation(res.into(),weights().into(),|(r,w)|r*w).unwrap().into();
            let update=match super::try_solve_least_squares(wjac.clone(),wres) {
                Some(update) => update,
                None => { return Err(OptimizationError::MatrixNotFullRank(wjac, x)); }
            };
            if &update.clone().norm() < opts.target_cost() { break; }
            x=from_dvec(into_dvec(x).clone().try_add(update).unwrap());
            iter+=1;
        }
        if &iter == opts.max_iter() {
            return Err(OptimizationError::MaximalIteration(iter));
        }
        Ok(x)
    }
}


// #[derive(Clone)]
// pub struct ProblemInclDerivative<F:Scalar,
//                                  X:Parameters<F>,
//                                  Y:Parameters<F>,
//                                  Func:'static+Fn(&X) -> Y,
//                                  DF:'static+Fn(&X) -> MatrixDyn<F>> {
//     problem:Problem<F,X,Y,Func>,
//     derivative: Box<DF>
// }

// impl<F:Scalar,
//      X:Parameters<F>+NumberOfDegreesOfFreedom<F>,
//      Y:Parameters<F>,
//      Func: 'static+Fn(&X) -> Y,
//      DF:'static+Fn(&X) -> MatrixDyn<F>> ProblemInclDerivative<F,X,Y,Func,DF> {

//     pub fn regualarize(self, reg_const:F::RealType) -> ProblemInclDerivative<
//         F,
//         X,
//         Concatenated<X,Y>,
//         impl 'static+Fn(&X) -> Concatenated<X,Y>,
//         impl 'static+Fn(&X) -> MatrixDyn<F>> {
//         let derivative=move |x:&X|{
//                  let mx=MatrixDyn::<F>::identity(X::NDOFS);
//                  let my=MatrixDyn::<F>::from((self.derivative)(x));
//                  MatrixDyn::try_vstack(mx, my).unwrap()
//              };
//         ProblemInclDerivative{problem:self.problem.regularize(reg_const),
//                               derivative:derivative.into()}
//     }

//     pub fn solve(&self) -> Result<X, OptimizationError<F, X>> {
//         self.problem.solve_with_der(|x:&X|(self.derivative)(x))
//     }
// }
