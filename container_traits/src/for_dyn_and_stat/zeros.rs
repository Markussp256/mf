use num_traits::Zero;

pub trait Zeros<Index,T> {
    fn zeros(size:Index) -> Self where T:Zero;
}

impl<T> Zeros<usize,T> for Vec<T> {
    fn zeros(size:usize) -> Self where T : Zero {
        std::iter::repeat_with(||T::zero())
            .take(size)
            .collect()
    }
}

impl<T, const N:usize> Zeros<usize,T> for [T;N] {
    fn zeros(size:usize) -> Self where T:Zero {
        assert_eq!(size,N);
        std::array::from_fn(|_|T::zero())
    }
}

// impl<Index:Clone,T:AnyZeros<Index>> Zeros<(usize,Index)> for Vec<T> {
//     fn zeros(size:(usize,Index)) -> Self {
//         std::iter::repeat_with(||T::any_zeros(size.1.clone()))
//             .take(size.0)
//             .collect()
//     }
// }

// impl<Index:Clone,T:Zeros<Index>, const N:usize> Zeros<(usize,Index)> for [T;N] {
//     fn zeros(size:(usize,Index)) -> Self {
//         assert_eq!(size.0,N);
//         std::array::from_fn(|_|T::zeros(size.1.clone()))
//     }
// }