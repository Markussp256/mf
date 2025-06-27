// we can not implement Zero for Array, because of the orphan rule
// but we can define our own trait

pub trait Zero {
    fn zero() -> Self;
}

// for array
impl<T:Zero, const N:usize> Zero for [T;N] {
    fn zero() -> Self {
        std::array::from_fn(|_|T::zero())
    }
}
