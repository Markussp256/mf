// use math_traits::{algebra::{Affinespace, NumberOfDegreesOfFreedom, Parameters, Scalarproduct}, geometrie::Submanifold};
// use nalgebra::DMatrix;
// use super::{OptimizationOptions, OptimizationError};

// pub fn riemannian_newton<M:Clone+Submanifold>(f:& impl Fn(&M::AmbientSpace) -> f64,
//                                               df:Option<impl Fn(&M::AmbientSpace) -> <M::AmbientSpace as Affinespace>::V>,
//                                               first_guess:M,
//                                               opts:Option<OptimizationOptions>) -> Result<M, OptimizationError<M>> 
//     where 
//     M::AmbientSpace : Parameters+NumberOfDegreesOfFreedom,
//     <M::AmbientSpace as Affinespace>::V : Scalarproduct<Output=f64> {
//     let opts: OptimizationOptions = opts.unwrap_or_default();
//     let mut x=first_guess.clone();
//     let mut iter:u8=0;
//     while &iter < opts.max_iter() {
//         let grad=|xx:&M::AmbientSpace|super::jacobian::jacobian(f, xx, Some(opts.fd().clone())).column(0);
//         let xe=x.embedding();
//         let gradx=grad(&xe);
//         let mut hessx=super::jacobian::jacobian(&grad, &xe, Some(opts.fd().clone()));
//         hessx-=x.riemannian_hessian_correction(|v|v.scalar_product(gradx))*DMatrix::identity(hessx.nrows(),hessx.ncols());
//         // restrict to tangent space
//         let f=|tv:&Vec<f64>|{
//             let tv=x.tangent_space(tv)-xe;

//         }


//         iter+=1;
//     }
//     if &iter == opts.max_iter() {
//         return Err(OptimizationError::MaximalIteration(iter));
//     }
//     Ok(x)
// }