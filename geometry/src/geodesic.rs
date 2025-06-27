use std::ops::{Mul,Div};
use num_traits::Pow;

use super::trafos::SE3;
use super::primitives::Frame;

pub type FrameGeodesic=Geodesic<Frame, SE3>;

#[derive(Clone, Debug)]
pub struct Geodesic<T, R> {
    start:T,
    ratio:R
}

impl<T:Clone+Div<Output=R>, R:Mul<T,Output=T>+Clone> Geodesic<T, R> {
    pub fn new(start:T, end:T) -> Self {
        Self{start:start.clone(),
             ratio:end/start}
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn end(&self) -> T {
        self.ratio.clone()*self.start.clone()
    }

    pub fn ratio(&self) -> R {
        self.ratio.clone()
    }

    pub fn eval<F>(&self, t:F) -> T where R:Pow<F, Output=R> {
        self.ratio.clone().pow(t)*self.start.clone()
    }
}