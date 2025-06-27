use core::panic;
use container_traits::IntoSum;
use num_traits::Zero;
use std::ops::{Add, Mul};

use algebra_traits::{Exp, MultiplicativeGroup, Scalar, Tolerance, TryLog, Vectorspace};
use geometry_traits::LieGroup;

// tangent vector is received by applying parallel transport to vector

#[derive(Clone,
        PartialEq)]
pub struct TangentBundle<M, LogM> {
    point: M,
    vector: LogM,
}

impl<M: PartialEq, LogM: Add<Output = LogM>> TangentBundle<M, LogM> {
    fn try_add(self, rhs: Self) -> Option<Self> {
        if self.point == rhs.point {
            Some(TangentBundle {
                point: self.point,
                vector: self.vector + rhs.vector,
            })
        } else {
            None
        }
    }

    fn try_sum(mut iter: impl Iterator<Item=Self>) -> Option<Self> {
        let mut res=iter.next()?;
        while let Some(elem)=iter.next() {
            res=res.try_add(elem)?;
        }
        Some(res)
    }

}

impl<M, LogM: Mul<F,Output=LogM>, F> Mul<F> for TangentBundle<M, LogM> where LogM : Mul<F> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        TangentBundle {
            point: self.point,
            vector: self.vector * rhs.into(),
        }
    }
}


// impl<M:NumberOfDegreesOfFreedom, LogM> NumberOfDegreesOfFreedom for TangentBundle<M, LogM> {
//     const NDOFS: usize=2*M::NDOFS;
// }

// impl<M:Manifold, LogM:Parameters+Clone+Vectorspace<f64>> Manifold for TangentBundle<M, LogM> {
//     fn loc_try_from_parameters(&self, x: Vec<f64>) -> Self {
//         Self { point: M::loc_try_from_parameters(&self.point, x[..x.len()/2].to_vec()),
//                vector: self.vector.clone()+LogM::try_from_parameters(x[x.len()/2..].to_vec()).unwrap() }
//     }

//     fn loc_parameters(&self, rhs: &Self) -> Vec<f64> {
        
//         let v0=M::loc_parameters(&self.point, &rhs.point);
//         let v1=(rhs.vector.clone()-self.vector.clone()).parameters();
//         utils::vec::concat(v0, v1)
//     }
// }

impl<M:Clone, LogM: Zero> TangentBundle<M, LogM> {
    pub fn proj_m(self) -> M {
        self.point
    }
    pub fn zero_at(a: &M) -> Self {
        TangentBundle {
            point: a.clone(),
            vector: LogM::zero(),
        }
    }
}

impl<M: Mul<Output=M>, LogM: Exp<Output=M>> Exp for TangentBundle<M, LogM> {
    type Output=M;
    fn exp(self) -> M {
        self.point * self.vector.exp()
    }
}


impl<M:Clone+MultiplicativeGroup+TryLog<Output=LogM>, LogM:Clone> TangentBundle<M, LogM> {
    fn try_log(a:&M, b:&M) -> Option<TangentBundle<M, LogM>> {
        let vector = (a.clone() * b.clone().inv()).try_log().ok()?;
        Some(TangentBundle {
            point: a.clone(),
            vector,
        })
    }
}

// there is a type Geodesic
// impl<M: Clone + MultiplicativeGroup + Pow<f64, Output = M>> Geodesic<f64> for M {
//     fn geodesic(p0: &Self, p1: &Self, t: f64) -> Self {
//         p0.clone() * (p0.clone().inv() * p1.clone()).pow(t)
//     }
// }

pub fn weighted_average
    <M    : Clone + PartialEq + LieGroup<f64,E> + TryLog<Output=LogM>,
     E,
     LogM : Clone + Mul<F,Output=LogM>+Vectorspace<F>+Exp<Output=M>,
     F    : Scalar>(
    wpts: Vec<(F::RealType, M)>,
    init: Option<M>,
) -> M {
    // check input
    let w: Vec<F::RealType> = wpts.iter().map(|(w, _)| w.clone()).collect();
    if !w.into_sum().is_close_to_one() {
        panic!("sum of weights must be 1")
    }

    let mut result: M =
    init.unwrap_or(wpts.iter()
                                .max_by(|a,b|a.0.partial_cmp(&b.0).unwrap())
                                .unwrap()
                                .1
                                .clone());

    // compute result
    let wpts:Vec<(F,M)>=wpts.into_iter().map(|(w,m)|(w.into(),m)).collect();

    for _ in 1..5 {
        result = TangentBundle::<M,LogM>::try_sum(wpts.iter()
                                                            .map(|(w, pt)| TangentBundle::<M,LogM>::try_log(&result, pt).unwrap() * w.clone()))
                        .unwrap()
                        .exp(); //.expect("points too far away, logarithm map can not be applied"))
    }
    result
}
