use crate::{matrix_operations::Transpose, transpose::Transposed};
use algebra::VectorGeneric;
use container_traits::{
    IndexOutOfBoundsError, LinearContainer, LinearContainerConstruct, LinearContainerDynamic, LinearContainerTryConstruct, LinearContainerView, LinearContainerViewMut};

use container::{Concatenated,ContainerSparse,ContainerSparseView};


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

        pub trait [<$r_or_c VectorViewMut>] : [<$r_or_c VectorView>] + LinearContainerViewMut {}
        impl<S:[<$r_or_c VectorViewMut>]+LinearContainerViewMut> [<$r_or_c VectorViewMut>] for S {}

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
    fn transpose(&self) -> Self::Output where Self:Clone {
        self.clone()
            .into_transpose()
    }

    fn into_transpose(self) -> Self::Output {
        let (a,b)=self.into_parts();
        Concatenated::new(
            a.into_transpose(),
            b.into_transpose())
    }
}

impl<C:LinearContainerView> Transpose for VectorGeneric<C> {
    type Output=Transposed<Self>;
    fn transpose(&self) -> Self::Output where Self : Clone {
        Transposed::new(self.clone())
    }

    fn into_transpose(self) -> Self::Output {
        Transposed::new(self)
    }
}

impl<C:LinearContainerView> ColVectorView for VectorGeneric<C> {}

macro_rules! impl_row_col_view {
    ($name:ident, $other:ident, $tr:ident) => {
        #[derive(Clone, Debug,
            container_derive::ContainerView)]
        pub struct $name<'a,T>(ContainerSparseView<'a,usize,T>);


        impl<'a,T> $name<'a,T> {
            pub fn new(default:&'a T, size:usize) -> Self {
                Self(ContainerSparseView::new(default,size))
            }

            pub fn insert(&mut self, key:usize, value:&'a T) -> Result<Option<&'a T>,IndexOutOfBoundsError<usize>> {
                self.0
                    .insert(key,value)
            }
        }

        impl<'a,T> $tr for $name<'a,T> {}

        impl<'a,T> Transpose for $name<'a,T> {
            type Output = $other<'a,T>;
            fn transpose(&self) -> Self::Output where Self : Clone {
                $other(self.clone().0)
            }

            fn into_transpose(self) -> Self::Output {
                $other(self.0)
            }
        }
    };
}
impl_row_col_view!(SparseRowView, SparseColView, RowVectorView);
impl_row_col_view!(SparseColView, SparseRowView, ColVectorView);

macro_rules! impl_row_col {
    ($name:ident, $other:ident, $tr:ident) => {
        #[derive(Clone, Debug,
            container_derive::Container)]
        pub struct $name<T>(ContainerSparse<usize,T>);


        impl<T> $name<T> {
            pub fn new(default:T, size:usize) -> Self {
                Self(ContainerSparse::new(default,size))
            }

            pub fn insert(&mut self, key:usize, value:T) -> Result<Option<T>,IndexOutOfBoundsError<usize>> {
                self.0
                    .insert(key,value)
            }
        }

        impl<T> $tr for $name<T> {}

        impl<T> Transpose for $name<T> {
            type Output = $other<T>;
            fn transpose(&self) -> Self::Output where Self : Clone {
                $other(self.clone().0)
            }

            fn into_transpose(self) -> Self::Output {
                $other(self.0)
            }
        }
    };
}
impl_row_col!(SparseRow, SparseCol, RowVectorView);
impl_row_col!(SparseCol, SparseRow, ColVectorView);
