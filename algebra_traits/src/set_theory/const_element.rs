

pub trait ConstElement {
    const ELEMENT:Self;
}

impl<T:ConstElement, const N:usize> ConstElement for [T;N] {
    const ELEMENT:Self=[T::ELEMENT;N];
}