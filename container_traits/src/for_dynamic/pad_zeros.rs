use num_traits::Zero;
use crate::Len;

pub trait PadZeros<T:Zero> {
    fn pad_zeros(&mut self, total_len:usize);
}

impl<T:Zero,S:Extend<T>+Len> PadZeros<T> for S {
    fn pad_zeros(&mut self, total_len:usize) {
        let current_len=self.len();
        if current_len < total_len {
            let z_iter=std::iter::repeat_with(T::zero);
            self.extend(z_iter.take(total_len-current_len))
        }
    }
}