use crate::{ColVector, ColVectorView, RowVector, RowVectorView};

use container_traits::*;

#[derive(
    Clone,
    Debug,
    container_derive::Container,
    algebra_derive::ScalarContainer)]
pub struct View<'a,C> (&'a C);


// impl<'a,C:ItemT> ItemT for View<'a,C> {
//     type T=C::T;
// }

impl<'a,
     F,
     C : ColVector<T=&'a F>> ColVectorView for View<'a,C> {
}

impl<'a,
     F,
     C : RowVector<T=&'a F>> RowVectorView for View<'a,C> {
}