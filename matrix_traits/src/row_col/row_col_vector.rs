use crate::{matrix_operations::Transpose, transpose::Transposed};
use algebra::VectorGeneric;
use container_traits::{Concatenated, LinearContainer, LinearContainerDynamic, LinearContainerMut, LinearContainerConstruct, LinearContainerTryConstruct};

macro_rules! row_col_traits {
    ($r_or_c:ident) => {
        paste::paste!(
        pub trait [<$r_or_c Vector>] : Transpose + LinearContainer {}

        pub trait [<$r_or_c VectorAnyConstruct>] : [<$r_or_c Vector>] + LinearContainerTryConstruct {}
        impl<S:[<$r_or_c Vector>]+LinearContainerTryConstruct> [<$r_or_c VectorAnyConstruct>] for S {}

        pub trait [<$r_or_c VectorConstruct>] : [<$r_or_c Vector>] + LinearContainerConstruct {}
        impl<S:[<$r_or_c Vector>]+LinearContainerConstruct> [<$r_or_c VectorConstruct>] for S {}

        pub trait [<$r_or_c VectorMut>] : [<$r_or_c Vector>] + LinearContainerMut {}
        impl<S:[<$r_or_c Vector>]+LinearContainerMut> [<$r_or_c VectorMut>] for S {}

        pub trait [<$r_or_c VectorDynamic>] : [<$r_or_c Vector>] + LinearContainerDynamic {}
        impl<S:[<$r_or_c Vector>]+LinearContainerDynamic> [<$r_or_c VectorDynamic>] for S {}

        impl<T,
             A:[<$r_or_c Vector>]<T=T>,
             B:[<$r_or_c Vector>]<T=T>> [<$r_or_c Vector>] for Concatenated<A,B> {}
        );
    };
}
row_col_traits!(Row);
row_col_traits!(Col);


impl<A : Transpose<Output=AT>,AT,
     B : Transpose<Output=BT>,BT> Transpose for Concatenated<A,B> {
    type Output = Concatenated<AT,BT>;
    fn transpose(self) -> Self::Output {
        let (a,b)=self.into_parts();
        Concatenated::new(
        a.transpose(),
        b.transpose())
    }
}



impl<C:LinearContainer> Transpose for VectorGeneric<C> {
    type Output=Transposed<Self>;

    fn transpose(self) -> Self::Output {
        Transposed::new(self)
    }
}

impl<C:LinearContainer> ColVector for VectorGeneric<C> {}