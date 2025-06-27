pub trait OneElement<T> {
    fn one_element(t:T) -> Self;
}


impl<T> OneElement<T> for Vec<T> {
    fn one_element(t:T) -> Vec<T> {
        vec![t]
    }
}