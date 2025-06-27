pub trait NumberOfDegreesOfFreedom<F> {
    const NDOFS: usize;
}

impl<T, const N: usize> NumberOfDegreesOfFreedom<T> for [T; N] {
    const NDOFS: usize = N;
}


// use core::fmt;

// #[derive(Debug, Clone)]
// pub enum NDynamicContainer {
//     Fixed(usize),
//     Undefined
// }

// impl std::fmt::Display for NDynamicContainer {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Fixed(s) => write!(f,"{} dynamic containers",s),
//             Self::Undefined => write!(f,"undefined number of dynamic containers")
//         }
//     }
// }

// #[derive(Debug)]
// pub enum DofsError {
//     IteratorNotLargeEnough(usize, usize),
//     MoreThanOneDynamicContainer(NDynamicContainer),
// }

// impl fmt::Display for DofsError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             DofsError::IteratorNotLargeEnough(iterlen, n_static) =>
//             write!(f,"iterator should have at least {} but it has only {}<{} elements",n_static, iterlen, n_static),
//             DofsError::MoreThanOneDynamicContainer(n_dynamic_container) =>
//             write!(f, "for types with undefined or more than one dynamic container,
//                         we can not distribute the remaining iterator items if there are more than needed for the static, {}", n_dynamic_container),
//         }
//     }
// }

// impl std::error::Error for DofsError {}

// pub struct Dofs {
//     n_static:usize,
//     n_dynamic_container:NDynamicContainer,
// }

// impl Dofs {
//     // computes how many items of the iterator should be used for the dynamic container
//     fn n_dyn(&self, iterlen:usize) -> Result<Option<usize>,DofsError> {
//         let checked_diff=iterlen.checked_sub(self.n_static);
//         match self.n_dynamic_container {
//             NDynamicContainer::Fixed(0) => Ok(None),
//             NDynamicContainer::Fixed(1) => checked_diff.ok_or(DofsError::IteratorNotLargeEnough(iterlen,self.n_static)),
//             dc => if let checked_diff = Some(0) {
//                     Ok(Some(0))
//                  } else {
//                     Err(DofsError::MoreThanOneDynamicContainer(dc))
//                  }
//         }
//     }
// }
