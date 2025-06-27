use num_traits::Zero;
use algebra_traits::{Max, Nonnegative, Norm};
use container_traits::IntoIter;

// returns none if N == 0
pub fn try_max<T:Max>(a:impl IntoIter<T>) -> Option<T> {
    a.into_iterator()
     .reduce(T::into_max)
}

// empty array/vec has norm zero
pub fn max_norm<T:Norm<NormT=SO>, SO:Zero+Max>(a:impl IntoIter<T>) -> Nonnegative<SO> {
        a.into_iterator()
         .map(T::norm)
         .fold(Nonnegative::zero(),|cmax, new| 
            Nonnegative::into_max(cmax, new))
}