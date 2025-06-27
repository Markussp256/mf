use std::{cmp::PartialOrd, ops::{Add, Div, Sub}};

use super::{TryMin, TryMax};


#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Interval<T> {
    lb: T,
    ub: T,
}

impl<T> Interval<T> {
    pub fn ub(&self) -> &T {
        &self.ub
    }

    pub fn lb(&self) -> &T {
        &self.lb
    }

    pub fn bounds(&self) -> [&T;2] {
        [&self.lb,
         &self.ub]
    }
}

impl<T: PartialOrd> Interval<T> {
    pub fn try_new(lb: T, ub: T) -> Option<Self> {
        if lb < ub {
            Some(Interval { lb, ub })
        } else {
            None
        }
    }

    pub fn try_spanned_interval(vs: Vec<T>) -> Option<Self>
    where
        T: Clone,
    {
        let min = vs.iter().min_by(|a, b| a.partial_cmp(b).unwrap())?;
        let max = vs.iter().max_by(|a, b| a.partial_cmp(b).unwrap())?;
        Self::try_new(min.clone(), max.clone())
    }

    pub fn map<T2: PartialOrd, F: Fn(T) -> T2>(self, f: F) -> Interval<T2> {
        Interval::<T2> {
            lb: f(self.lb),
            ub: f(self.ub),
        }
    }

    pub fn contains(&self, v: &T) -> bool {
        &self.lb <= v && v <= &self.ub
    }

    pub fn strictly_contains(&self, v: &T) -> bool {
        &self.lb < v && v < &self.ub
    }

    pub fn try_intersect(self, rhs: Self) -> Option<Self>
    where
        T: TryMin+TryMax,
    {
        let lb = self.lb.try_max(rhs.lb)?;
        let ub = self.ub.try_min(rhs.ub)?;
        Self::try_new(lb, ub)
    }
}

impl<T> Interval<T> {
    pub fn length<V>(&self) -> V where T:Clone+Sub<Output=V> {
        self.ub.clone() - self.lb.clone()
    }

    pub fn sample_n<F:From<u32>, V:Clone+Div<F, Output=V>>(&self, n:u32) -> Vec<T> where T:Clone+Sub<Output=V>+Add<V,Output=T> {
        match n {
            0 => Vec::<T>::new(),
            1 => vec![self.lb.clone()],
            _ => {
                let d:V=self.length()/F::from(n-1);
                let mut res=Vec::with_capacity(n as usize);
                let mut current=self.lb().clone();
                for _ in 0..n {
                    res.push(current.clone());
                    current=current+d.clone();
                }
                res
            }
        }
    }

    // pub fn sample_d<F, V:Vectorspace1d<F=F>>(&self, d:V) -> Vec<T> where T:Clone+Sub<Output=V> {
    //     let n=(self.length()/d).ceil() as u32+1;
    //     self.sample_n::<F,V>(n)
    // }
}

#[test]
fn test_sample_d() {
    let iv=Interval::try_new(2.3, 3.4).unwrap();
    assert_eq!(vec![2.3,3.4],iv.sample_n::<f64, f64>(2))
}

#[test]
fn test_spanned_by() {
    let vs=vec![1.2, -3.1, 5.3];
    let iv=Interval::try_spanned_interval(vs);
    assert_eq!(iv, Interval::try_new(-3.1, 5.3));
}


#[derive(Debug)]
pub enum IntervalBuilderError {
    LowerBoundNotSet,
    UpperBoundNotSet,
    LowerAndUpperBoundNotSet,
    LowerBoundIsLargerOrEqualToUpperBound
}

#[derive(Clone, Copy, Debug)]
pub struct IntervalBuilder<T: PartialOrd> {
    lb:Option<T>,
    ub:Option<T>
}

impl<T: PartialOrd> IntervalBuilder<T> {

    pub fn lb(& mut self, t:T) -> & mut Self {
        self.lb=Some(t);
        self
    }

    pub fn ub(& mut self, t:T) -> & mut Self {
        self.ub=Some(t);
        self
    }

    pub fn build(self) -> Result<Interval<T>,IntervalBuilderError> {
        match (self.lb,self.ub) {
            (None, None) => Err(IntervalBuilderError::LowerAndUpperBoundNotSet),
            (None, Some(_)) => Err(IntervalBuilderError::LowerBoundNotSet),
            (Some(_), None) => Err(IntervalBuilderError::UpperBoundNotSet),
            (Some(lb), Some(ub)) => {
                if lb >= ub {
                    Err(IntervalBuilderError::LowerBoundIsLargerOrEqualToUpperBound)
                } else {
                    Ok(Interval::try_new(lb, ub).unwrap())
                }
            },
        }
    }
}

impl<T:PartialOrd> Default for IntervalBuilder<T> {
    fn default() -> Self {
        Self { lb: None, ub: None }
    }
}


#[test]
fn test_interval() {
    let iv = Interval::try_new(1, 3).unwrap();
    assert!(iv.contains(&2));
    assert!(!iv.contains(&4));
}

#[test]
fn test_interval_wrong_order() {
    let oiv: Option<Interval<i32>> = Interval::try_new(2, 1);
    assert!(oiv.is_none());
}
