// use nalgebra::Scalar;

use algebra_traits::Interval;
use splines::{interpolate::Interpolator, spline, Interpolate, Interpolation};

use container_traits::Parameter;

//pub use splines::{Key,Interpolation};
// #[derive(Clone)]
// pub enum InterpolationType{
//     Linear,
//     Cosine,
//     CatmullRom,
// }

#[derive(Clone)]
pub struct Knot<T, V> {
    t: T,
    v: V,
}

impl<T, V> Knot<T, V> {
    pub fn new(t: T, v: V) -> Self {
        Self { t, v }
    }
    pub fn t(&self) -> &T {
        &self.t
    }
    pub fn v(&self) -> &V {
        &self.v
    }
}

pub trait SplineExtension<T, V> {
    // must be valid inside the region
    fn sample(&self, t:T) -> Option<V>;

    // lower bound
    fn lb(&self) -> Option<(T, V)>;

    // upper bound
    fn ub(&self) -> Option<(T, V)>;
    
    // provided
    // also gives value at the boundary
    fn sample_ext(&self, t:T) -> Option<V> where T : PartialEq {
        let (lb,lb_val)=self.lb()?;
        let (ub,ub_val)=self.ub()?;
        match t {
            t if t == lb => Some(lb_val),
            t if t == ub => Some(ub_val),
            t => self.sample(t)
        }
    }
}

impl<T:Interpolator, V:Interpolate<T>> SplineExtension<T, V> for splines::Spline<T, V> {
    fn sample(&self, t:T) -> Option<V> {
        splines::Spline::sample(&self, t)
    }

    fn lb(&self) -> Option<(T, V)> {
        self.get(0)
            .map(|key|(key.t.clone(), key.value.clone()))
    }

    fn ub(&self) -> Option<(T, V)> {
        let len=self.len();
        if len == 0 {
            None
        } else {
        self.get(len-1)
            .map(|key|(key.t.clone(), key.value.clone()))
        }
    }
}

fn to_splines_keys<T: Parameter<f64>, V: Clone>(
    vs: Vec<Knot<T, V>>,
    it: Interpolation<f64, V>,
) -> Vec<splines::Key<f64, V>> {
    vs.into_iter()
        .map(|k| splines::Key::new(k.t.into_parameter(), k.v, it.clone()))
        .collect()
}

#[derive(Clone, Debug)]
pub struct Spline<T, V> {
    input_interval: Interval<T>,
    sp: spline::Spline<f64, V>,
}

impl<T: Parameter<f64> + PartialOrd, V: Interpolate<f64>> Spline<T, V> {
    pub fn eval(self, t: T) -> V {
        assert!(self.input_interval.contains(&t));
        let tp=t.into_parameter();
        // spline will say its outside
        self.sp
            .sample_ext(tp)
            .expect(&format!("input value {} is outside the spline region [{},{}]",
                                  tp,
                                  self.sp.get(0).unwrap().t,
                                  self.sp.get(self.sp.len()-1).unwrap().t))
    }

    pub fn try_new(vk: Vec<Knot<T, V>>, i: Interpolation<f64, V>) -> Option<Self>
    where
        T: Clone,
    {
        let vs: Vec<T> = vk.iter().map(|ki| ki.t.clone()).collect();
        let input_interval = Interval::try_spanned_interval(vs)?;
        let ksn = to_splines_keys(vk, i);
        Some(Self {
            input_interval,
            sp: spline::Spline::from_vec(ksn),
        })
    }
}
