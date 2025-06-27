use core::fmt::Debug;
use std::{collections::HashMap, hash::Hash};

use container_traits::{for_static::NumberOfDegreesOfFreedom, Iter};

#[derive(Debug, Clone, PartialEq, Hash, Eq, serde::Serialize, serde::Deserialize)]
pub struct Monomial {
    exponents: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TotalDegree(usize);

impl TotalDegree {
    pub fn new(deg: usize) -> Self {
        Self(deg)
    }
    pub fn usize(&self) -> usize {
        self.0
    }
}

impl Monomial {
    pub fn new(exponents: Vec<usize>) -> Self {
        Monomial { exponents }
    }

    pub fn eval(&self, val: Vec<f64>) -> f64 {
        val.iter()
            .zip(self.exponents.iter())
            .fold(1.0, |accm, (&xi, &ei)| accm * xi.powi(ei as i32))
    }

    pub fn degree(&self) -> TotalDegree {
        TotalDegree(self.exponents.iter().sum())
    }

    pub fn append(&mut self, new: usize) {
        self.exponents.push(new);
    }

    fn all_up_to(deg: TotalDegree, nv: usize) -> HashMap<TotalDegree, Vec<Self>> {
        let mut result = HashMap::new();
        if nv == 1 {
            for d in 0..=deg.usize() {
                result.insert(TotalDegree(d), vec![Monomial::new(vec![d])]);
            }
            return result;
        }
        let prev: HashMap<TotalDegree, Vec<Self>> = Self::all_up_to(deg, nv - 1);
        for d in 0..=deg.usize() {
            let mut allvec: Vec<Monomial> = Vec::new();
            for i in 0..=d {
                let mut addvec = prev[&TotalDegree::new(d - i)].clone();
                for addveci in addvec.iter_mut() {
                    addveci.append(i);
                }
                allvec.extend(addvec);
            }
            result.insert(TotalDegree::new(d), allvec);
        }
        return result;
    }

    pub fn all_with(deg: TotalDegree, nv: usize) -> Vec<Self> {
        let all = Self::all_up_to(deg, nv);
        // println!("All keys: {:?}",all.keys());
        // println!("Key given: {:?}",deg);
        all[&deg].clone()
    }
}



#[test]
fn test_monomial() {
    let exponents = vec![1 as usize, 2 as usize, 0 as usize];
    let mono = Monomial::new(exponents);
    let val = vec![0.1, 0.2, 0.3];
    assert_eq!(0.1 * 0.2 * 0.2, mono.eval(val));
    assert_eq!(3, mono.degree().usize());

    // append
    let mut monomut = mono;
    monomut.append(3 as usize);
    let val = vec![0.1, 0.2, 0.3, 0.4];
    assert_eq!(0.1 * 0.2 * 0.2 * (0.4_f64).powi(3), monomut.eval(val));
    assert_eq!(6, monomut.degree().usize());
}

#[test]
fn test_all_with() {
    let v = Monomial::all_with(TotalDegree::new(3), 2);
    assert_eq!(4, v.len());
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MultivariatePoly<const DEG: usize, const NV: usize> {
    coeffs: HashMap<Monomial, f64>,
}

impl<const DEG: usize, const NV: usize> MultivariatePoly<DEG, NV> {
    pub fn all_monomials() -> Vec<Monomial> {
        let hashed = Monomial::all_up_to(TotalDegree::new(DEG), NV);
        let mut result = Vec::new();
        for i in 0..=DEG {
            result.extend_from_slice(&hashed[&TotalDegree::new(i)]);
        }
        result
    }

    pub fn new() -> Self {
        let coeffs=HashMap::new();
        Self{ coeffs }
    }

    pub fn from_coeffs(coeffs: HashMap<Monomial, f64>) -> Self {
        Self { coeffs }
    }

    pub fn eval(&self, x: [f64; NV]) -> f64 {
        self.coeffs
            .iter()
            .map(|(e, c)| c * e.eval(x.clone().to_vec()))
            .sum()
    }

    pub fn getcoeff(&self, m: Monomial) -> f64 {
        self.coeffs[&m]
    }

    pub fn into_higher_deg_poly<const DEGPLUS1: usize>(self) -> MultivariatePoly<DEGPLUS1, NV> {
        assert_eq!(DEGPLUS1, DEG + 1);
        let mut coeffs = self.coeffs.clone();
        let monomnew = Monomial::all_with(TotalDegree::new(DEG + 1), NV);
        coeffs.extend(monomnew.into_iter().map(|m| (m, 0.0)));
        <MultivariatePoly<DEGPLUS1, NV>>::from_coeffs(coeffs)
    }
}

impl<const NV: usize, const DEG: usize> Iter<f64> for MultivariatePoly<DEG, NV> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a f64> where f64 : 'a {
        self.coeffs
            .iter()
            .map(|(_, c)| c)
    }
}

impl<const NV: usize, const DEG: usize> IntoIterator for MultivariatePoly<DEG, NV> {
    type Item=(Monomial,f64);
    type IntoIter = std::collections::hash_map::IntoIter<Monomial,f64>;
    fn into_iter(self) -> Self::IntoIter {
        self.coeffs
            .into_iter()
    }
}

impl<const NV: usize, const DEG: usize> FromIterator<(Monomial,f64)> for MultivariatePoly<DEG, NV> {
    fn from_iter<I:IntoIterator<Item=(Monomial,f64)>>(iter:I) -> Self {
        let mut coeffs=HashMap::new();
        for (m,v) in iter {
            match coeffs.get_mut(&m) {
                None => { assert!(coeffs.insert(m,v).is_none()); },
                Some(val) => *val += v
            }
        }
        Self{ coeffs }
    }
}

#[test]
fn test_from_to_parameters_polynomial() {
    let a0 = 1.0;
    let a1 = 2.0;
    let a2 = 3.0;
    let a3 = 4.0;
    let p=
        MultivariatePoly::<1, 3>::from_iter(
            MultivariatePoly::<1,3>::all_monomials()
                .into_iter()
                .zip([a0,a1,a2,a3].into_iter()));
    let x1 = 1.3;
    let x2 = -2.2;
    let x3 = 4.2;
    assert!((p.eval([x1, x2, x3]) - (a0 + a1 * x1 + a2 * x2 + a3 * x3)).abs() < 1e-6);
}

macro_rules! impl_NDOFS_for_mpoly {
    ($nv:tt) => {
        impl<const DEG: usize> NumberOfDegreesOfFreedom<f64> for MultivariatePoly<DEG, $nv> {
            const NDOFS: usize = $crate::NChooseK!($nv + DEG, $nv);
        }
    };
}

impl_NDOFS_for_mpoly!(1);
impl_NDOFS_for_mpoly!(2);
impl_NDOFS_for_mpoly!(3);
impl_NDOFS_for_mpoly!(4);
impl_NDOFS_for_mpoly!(5);
impl_NDOFS_for_mpoly!(6);

impl<const N: usize, const NV: usize> std::ops::Neg for MultivariatePoly<N, NV> {
    fn neg(self) -> Self {
        Self::from_iter(self.into_iter().map(|(c,v)|(c,-v)))
    }
    type Output = Self;
}

impl<const N: usize, const NV: usize> std::ops::Add for MultivariatePoly<N, NV> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_iter(
            self.into_iter()
                .chain(rhs.into_iter())
        )
    }
}

impl<const N: usize, const NV: usize> std::ops::Sub for MultivariatePoly<N, NV> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self+(-rhs)
    }
}

impl<const N: usize, const NV: usize> std::ops::Mul<f64> for MultivariatePoly<N, NV> {
    fn mul(self, f: f64) -> Self {
        Self::from_iter(self.into_iter().map(|(e, c)| (e, c * f)))
    }
    type Output = Self;
}

impl<const N: usize, const NV: usize> std::ops::Div<f64> for MultivariatePoly<N, NV> {
    fn div(self, f: f64) -> Self {
        Self::from_iter(self.into_iter().map(|(e, c)| (e, c / f)))
    }
    type Output = Self;
}

impl<const N: usize, const NV: usize> num_traits::Zero for MultivariatePoly<N, NV> {
    fn zero() -> Self {
        Self::new()
    }

    fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|z| z.1.is_zero())
    }
}
