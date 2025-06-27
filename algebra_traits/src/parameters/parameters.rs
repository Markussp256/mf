
use std::ops::Sub;
use utils::iter::next_chunk;

pub trait Parameters<F = f64>: Clone {
    // required methods
    fn parameters(&self) -> Vec<F>;

    fn try_from_iter<I: Iterator<Item = F>>(iter: &mut I) -> Option<Self>;

    // provided Method
    fn try_from_parameters(vs: Vec<F>) -> Option<Self>
    {
        let mut vs_iter=vs.into_iter();
        Self::try_from_iter(&mut vs_iter)
    }
}


impl<T: Parameters<F>, F> Parameters<F> for Vec<T> {
    fn parameters(&self) -> Vec<F> {
        self.into_iter()
            .map(|t|t.parameters())
            .flatten()
            .collect()
    }

    fn try_from_iter<I: Iterator<Item = F>>(iter: &mut I) -> Option<Self> {
        Some(std::iter::from_fn(||T::try_from_iter(iter))
                    .collect())
    }
}

impl<T: Parameters<F>, F, const N: usize> Parameters<F> for [T; N] {
    fn parameters(&self) -> Vec<F> {
        self.iter()
            .flat_map(<T as Parameters<F>>::parameters)
            .collect()
    }

    fn try_from_iter<I: Iterator<Item = F>>(iter: &mut I) -> Option<Self> {
        next_chunk(&mut std::iter::from_fn(|| {
            <T as Parameters<F>>::try_from_iter(iter)
        })).ok()
        // <[T;N] as Parameters::<T>>::try_from_iter(& mut sub_iter)
    }
}


pub trait ParametersCheck<F=f64> : Parameters<F> {
    fn bijectivity_error_of_parameters(vs: Vec<F>) -> Option<Vec<F>>
    where
        F: Clone + Sub<Output = F>,
    {
        let res = Self::try_from_parameters(vs.clone())?.parameters();
        Some(
            vs.into_iter()
                .zip(res.into_iter())
                .map(|(v, r)| v - r)
                .collect(),
        )
    }

    fn bijectivity_error(self) -> Option<Vec<F>>
    where
        F: Clone + Sub<Output = F>,
    {
        Self::bijectivity_error_of_parameters(self.parameters())
    }
}


#[cfg(feature = "nalgebra_support")]
impl Parameters for nalgebra::DVector<f64> {
    fn parameters(&self) -> Vec<f64> {
        self.iter().cloned().collect()
    }

    fn try_from_iter<I: Iterator<Item = f64>>(iter: &mut I) -> Option<Self> {
        Some(iter.collect::<Vec<f64>>().into())
    }
}

#[cfg(feature = "nalgebra_support")]
impl<const N: usize> Parameters for nalgebra::SVector<f64, N> {
    fn parameters(&self) -> Vec<f64> {
        self.iter().cloned().collect()
    }

    fn try_from_iter<I: Iterator<Item = f64>>(iter: &mut I) -> Option<Self> {
        let vs: Vec<f64> = iter.collect();
        if N == vs.len() {
            Some(Self::from_fn(|i, _j| vs[i]))
        } else {
            None
        }
    }
}