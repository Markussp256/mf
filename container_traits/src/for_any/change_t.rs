pub trait ChangeT<T2> {
    type Output;
}

impl<T,T2> ChangeT<T2> for Vec<T> {
    type Output = Vec<T2>;
}

impl<T,T2, const N:usize> ChangeT<T2> for [T;N] {
    type Output = [T2;N];
}