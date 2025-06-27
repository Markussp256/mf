use std::iter::Take;

use crate::IntoProduct;
use utils::iter::{IntoExactSizeIterator, WithExactSize};

pub struct ContainerIndexIterator<const N:usize> {
    size        : [usize;N],
    current     : [usize;N],
}

impl<const N:usize> ContainerIndexIterator<N> {
    // is infinite iterator
    fn new(size: [usize;N]) -> Self {
        let current=std::array::from_fn(|_|0);
        ContainerIndexIterator { size, current}
    }

    pub fn new_exact_size(size: [usize;N]) -> WithExactSize<Take<Self>> {
        let s=Self::new(size.clone());
        let len=size.into_iter().into_product();
        s.into_exact_size_iter(len)
    }
}

impl<const N:usize> Iterator for ContainerIndexIterator<N> {
    type Item = [usize;N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.iter().zip(
           self.size.iter()).any(|(c,s)|c >= s) {
            return None;
        }
        let rv=self.current.clone();

        // prepare for next
        for (ci, size) in self.current.iter_mut().zip(self.size.iter()).rev() {
            if *ci < size - 1 {
                *ci += 1;
                break;
            } else {
                *ci = 0;
            }
        }
        Some(rv)
    }
}

// impl<const N:usize> ExactSizeIterator for ContainerIndexIterator<N> {
//     fn len(&self) -> usize {
//         self.total_len-
//         self.current
//             .iter()
//             .zip(self.size.iter())
//             .fold(0,|acc,(c,s)| acc*s+c)
//     }
// }