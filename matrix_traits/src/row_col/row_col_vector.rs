use crate::{matrix_operations::Transpose, transpose::Transposed};
use algebra::VectorGeneric;
use container_traits::{
    Concatenated,
    LinearContainer,
    LinearContainerView,
    LinearContainerDynamic,
    LinearContainerMut,
    LinearContainerConstruct,
    LinearContainerTryConstruct};



macro_rules! row_col_traits {
    ($r_or_c:ident) => {
        paste::paste!(
        
        pub trait [<$r_or_c VectorView>] : LinearContainerView {}

        pub trait [<$r_or_c Vector>] : [<$r_or_c VectorView>]+Transpose + LinearContainer {}
        impl<S:[<$r_or_c VectorView>]+Transpose+LinearContainer> [<$r_or_c Vector>] for S {}

        pub trait [<$r_or_c VectorTryConstruct>] : [<$r_or_c Vector>] + LinearContainerTryConstruct {}
        impl<S:[<$r_or_c Vector>]+LinearContainerTryConstruct> [<$r_or_c VectorTryConstruct>] for S {}

        pub trait [<$r_or_c VectorConstruct>] : [<$r_or_c Vector>] + LinearContainerConstruct {}
        impl<S:[<$r_or_c Vector>]+LinearContainerConstruct> [<$r_or_c VectorConstruct>] for S {}

        pub trait [<$r_or_c VectorMut>] : [<$r_or_c VectorView>] + LinearContainerMut {}
        impl<S:[<$r_or_c VectorView>]+LinearContainerMut> [<$r_or_c VectorMut>] for S {}

        pub trait [<$r_or_c VectorDynamic>] : [<$r_or_c Vector>] + LinearContainerDynamic {}
        impl<S:[<$r_or_c Vector>]+LinearContainerDynamic> [<$r_or_c VectorDynamic>] for S {}

        impl<T,
             A:[<$r_or_c VectorView>]<T=T>,
             B:[<$r_or_c VectorView>]<T=T>> [<$r_or_c VectorView>] for Concatenated<A,B> {}
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



impl<C:LinearContainerView> Transpose for VectorGeneric<C> {
    type Output=Transposed<Self>;

    fn transpose(self) -> Self::Output {
        Transposed::new(self)
    }
}

impl<C:LinearContainerView> ColVectorView for VectorGeneric<C> {}