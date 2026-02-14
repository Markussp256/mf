use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, DeriveInput, Type, WherePredicate};
use quote::quote;

use derive_helper::{DeriveHelper, preprocessor::*};

fn preprocess4matrix(input:& mut DeriveInput) -> (proc_macro2::TokenStream, Punctuated<WherePredicate,Comma>, Type, Type) {
    let (generics, where_clause, [(ty, mut wt)]) = preprocess_no_impl(input);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    (generics, where_clause, ty, wt)
}

#[proc_macro_derive(MatrixView)]
pub fn matrix_view_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, where_clause, ty, wt) = preprocess4matrix(&mut input);
    let tr=quote!{matrix_traits::MatrixView};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            type RowView<'a>=<#wt as #tr>::RowView<'a> where #wt : 'a;
            type ColView<'a>=<#wt as #tr>::ColView<'a> where #wt : 'a;
            fn nrows(&self) -> usize {
                <#wt as #tr>::nrows(&self.0)
            }
            fn ncols(&self) -> usize {
                <#wt as #tr>::ncols(&self.0)
            }

            fn try_row_view<'a>(&'a self, i:usize) -> Result<Self::RowView<'a>,container_traits::IndexOutOfBoundsError<usize>> {
                <#wt as #tr>::try_row_view(&self.0,i)
            }

            fn try_col_view<'a>(&'a self, j:usize) -> Result<Self::ColView<'a>,container_traits::IndexOutOfBoundsError<usize>> {
                <#wt as #tr>::try_col_view(&self.0,j)
            }
        }
    }.into()
}

#[proc_macro_derive(Matrix)]
pub fn matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, where_clause, ty, wt) = preprocess4matrix(&mut input);
    let tr=quote!{matrix_traits::Matrix};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            type Row=<#wt as #tr>::Row;
            type Col=<#wt as #tr>::Col;
            fn into_rows(self) -> impl ExactSizeIterator<Item=Self::Row> {
                <#wt as #tr>::into_rows(self.0)
            }
            fn into_cols(self) -> impl ExactSizeIterator<Item=Self::Col> {
                <#wt as #tr>::into_cols(self.0)
            }
        }
    }.into()
}



#[proc_macro_derive(AlgebraMatrix)]
pub fn algebra_matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, where_clause, ty, wt) = preprocess4matrix(&mut input);
    let tr=quote!{matrix_traits::AlgebraMatrix};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            fn try_col_sc_prod(&self, j0:usize, j1:usize) -> Result<Self::T, container_traits::IndexOutOfBoundsError<usize>> where Self::T:Clone
            {
                <#wt as #tr>::try_col_sc_prod(&self.0,j0,j1)
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixNormal)]
pub fn matrix_normal_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, where_clause, ty, wt) = preprocess4matrix(&mut input);
    quote! {
        impl #generics matrix_traits::MatrixNormal for #ty where #wt : algebra_traits::Conjugate<Output=#wt>, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixTryConstruct)]
pub fn matrix_try_construct_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let (generics, where_clause, ty, wt) = preprocess4matrix(&mut input);
    let tr=quote!{matrix_traits::MatrixTryConstruct};
    let e=quote!{matrix_traits::MatrixConstructError};
    let tr_accept=     quote!{container_traits::TryAccept<(usize,usize),#wt::T,#e> };
    let tr_from_iter=  quote!{container_traits::AnyFromIterator<        #wt::T,#e> };
    let tr_try_map=    quote!{container_traits::TryMap<#wt::T,          #wt::T,#e> };
    let tr_try_from_fn=quote!{container_traits::TryFromFn<(usize,usize),#wt::T,#e>};

    quote! {
        impl #generics #ty where #wt : #tr, Self : #tr_accept,  #where_clause {
            pub fn try_new(m:#wt) -> Result<Self, #e> {
                <Self as #tr_accept>::try_accept(m.size(), |s|m.get(s).unwrap())?;
                Ok(Self(m))
            }
        }

        impl #generics #tr_from_iter for #ty where #wt : #tr, Self : #tr_accept, #where_clause {
            fn any_take_away<I:Iterator<Item=#wt::T>>(oref:Option<&Self>,iter: & mut I) -> Result<Self,#e> {
                Self::try_new(
                    <#wt as #tr_from_iter>::any_take_away(oref.map(|r|&r.0), iter)?)

            }
            container_traits::any_from_iter_impl!(#wt::T, #e);
        }

        impl #generics #tr_try_map for #ty where #wt : #tr, Self : #tr_accept, #where_clause {
            type Output=Self;
            fn try_map(self, f:impl Fn(#wt::T) -> #wt::T) -> Result<Self, #e> {
                Self::try_new(
                <#wt as #tr_try_map>::try_map(self.0,f)?)
            }
        }

        impl #generics #tr_try_from_fn for #ty where #wt : #tr, Self : #tr_accept, #where_clause {
            fn try_from_fn(size:(usize,usize), f:impl Fn((usize,usize)) -> #wt::T) -> Result<Self, #e> {
                Self::try_new(
                    <#wt as #tr_try_from_fn>::try_from_fn(size,f)?)
            }
        }

        impl #generics #tr for #ty where #wt : #tr, Self : #tr_accept, #where_clause {
            fn try_from_rows(rows:impl ExactSizeIterator<Item=Self::Row>) -> Result<Self,#e> {
                Self::try_new(
                    <#wt as #tr>::try_from_rows(rows)?)
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixDynamic)]
pub fn matrix_dynamic_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, where_clause, ty, wt) = preprocess4matrix(&mut input);
    let tr=quote!{matrix_traits::MatrixDynamic};
    quote! {
        impl #generics #tr for #ty where
            Self: matrix_traits::MatrixTryConstruct+AsMut<#wt>,
            #wt : #tr<T=#wt::T,Row=Self::Row,Col=Self::Col>,
            Self::Col : matrix_traits::row_col::ColVectorDynamic,
            Self::Row : matrix_traits::row_col::RowVectorDynamic,
            #where_clause {
            fn try_push_row(&mut self, row:Self::Row) -> Result<(),Self::Row> {
                <#wt as #tr>::try_push_row(self.as_mut(),row)
            }
            fn try_push_col(&mut self, col:Self::Col) -> Result<(),Self::Col> {
                 <#wt as #tr>::try_push_col(self.as_mut(),col)
            }

            fn pop_row(&mut self) -> Option<Self::Row> {
                 <#wt as #tr>::pop_row(self.as_mut())
            }
            fn pop_col(&mut self) -> Option<Self::Col> {
                <#wt as #tr>::pop_col(self.as_mut())
            }
        }
    }.into()
}

#[proc_macro_derive(IntoDynMatrix)]
pub fn into_dyn_matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, where_clause, ty, wt) = preprocess4matrix(&mut input);
    let tr=quote!{matrix_traits::IntoDynMatrix};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, <#wt as #tr>::Output : matrix_traits::FromMatrix<Self>, #where_clause {
            type Output=<#wt as #tr>::Output;
        }
    }.into()
}

// not can also be used for nonmatrices
#[proc_macro_derive(Transpose)]
pub fn transpose_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr = parse_quote!{matrix_traits::Transpose};
    let fn_name_into=parse_quote!{ into_transpose };
    let (generics, where_clause, [(ty, wt),(ty1, wt1)], implementation_into)=
    DeriveHelper::new(&mut input, &tr, &fn_name_into).extended1();

    quote! {
        impl #generics #tr for #ty where #(#wt : #tr<Output=#wt1>,)* #where_clause {
            type Output=#ty1;

            fn into_transpose(self) -> #ty1 {
                #implementation_into
            }
        }
    }.into()
}


#[proc_macro_derive(ClosedTranspose)]
pub fn closed_transpose_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr = quote!{matrix_traits::Transpose};
    let (generics, where_clause, ty, wt) =  preprocess4matrix(&mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr<Output=#wt>, #where_clause {
            type Output=Self;

            fn into_transpose(self) -> Self {
                Self(self.0
                         .into_transpose())
            }
        }
    }.into()
}


#[proc_macro_derive(IsEmpty)]
pub fn is_empty_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr = quote!{container_traits::IsEmpty};
    let (generics, where_clause, ty, _) =  preprocess4matrix(&mut input);
    quote! {

        impl #generics #tr for #ty where #where_clause {
            fn is_empty(&self) -> bool {
                <Self as matrix_traits::MatrixView>::matrix_dimensions(&self) == (0,0)
            }
        }
    }.into()
}

#[proc_macro_derive(StaticMatrix)]
pub fn static_matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr = quote!{matrix_traits::StaticMatrix};
    let tr_sqr=quote!{matrix_traits::SquareStaticMatrixView};
    let (generics, where_clause, ty, wt) =  preprocess4matrix(&mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            const M:usize=<#wt as #tr>::M;
            const N:usize=<#wt as #tr>::N;
        }

        impl #generics #tr_sqr for #ty where #wt : #tr_sqr, Self : matrix_traits::MatrixViewSquare, #where_clause {
            const M:usize=<#wt as #tr_sqr>::M;
        }
    }.into()
}


#[proc_macro_derive(PopRow)]
pub fn pop_row_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let type_name=input.ident.clone();
    let tr = quote!{matrix_traits::PopRow};
    let tr_try = quote!{matrix_traits::TryPopRow};
    let (generics, where_clause, [(ty, mut wt)])=
    preprocess_no_impl(& mut input);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            type Output=#type_name<<#wt as #tr>::Output>;
            fn pop_row(self) -> (#type_name<<#wt as #tr>::Output>, <Self as Matrix>::Row) {
                let (m,r)=self.0.pop_row();
                (#type_name(m),r)
            }
        }

        impl #generics #tr_try for #ty where #wt : #tr_try, #where_clause {
            type Output=#type_name<<#wt as #tr_try>::Output>;
            fn try_pop_row(self) -> Option<(#type_name<<#wt as #tr_try>::Output>, <Self as Matrix>::Row)> {
                self.0
                    .try_pop_row()
                    .map(|(m,r)|(#type_name(m),r))
            }
        }
    }.into()
}


#[proc_macro_derive(PopCol)]
pub fn pop_col_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let type_name=input.ident.clone();
    let tr = quote!{matrix_traits::PopCol};
    let tr_try = quote!{matrix_traits::TryPopCol};
    let (generics, where_clause, [(ty, mut wt)])=
    preprocess_no_impl(& mut input);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            type Output=#type_name<<#wt as #tr>::Output>;
            fn pop_col(self) -> (#type_name<<#wt as #tr>::Output>, <Self as Matrix>::Col) {
                let (m,r)=self.0.pop_col();
                (#type_name(m),r)
            }
        }

        impl #generics #tr_try for #ty where #wt : #tr_try, #where_clause {
            type Output=#type_name<<#wt as #tr_try>::Output>;
            fn try_pop_col(self) -> Option<(#type_name<<#wt as #tr_try>::Output>, <Self as Matrix>::Col)> {
                self.0
                    .try_pop_col()
                    .map(|(m,r)|(#type_name(m),r))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixVectorProduct)]
pub fn matrix_vector_product_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr = quote!{matrix_traits::MatrixVectorProduct<Rhs>};
    let tr_try = quote!{matrix_traits::TryMatrixVectorProduct<Rhs>};
    let tr_into = quote!{matrix_traits::IntoMatrixVectorProduct<Rhs>};
    let tr_try_into = quote!{matrix_traits::TryIntoMatrixVectorProduct<Rhs>};
    let (generics, where_clause, [(ty, mut wt)])=
    preprocess_no_impl_add_gen_types(& mut input, vec!["Rhs"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, Rhs : matrix_traits::ColVectorView, #where_clause {
            type Output=<#wt as #tr>::Output;
            fn matrix_vector_product(&self, rhs:&Rhs) -> <#wt as #tr>::Output {
                self.0
                    .matrix_vector_product(rhs)
            }
        }


        impl #generics #tr_into for #ty where #wt : #tr_into, Rhs : matrix_traits::ColVectorView, #where_clause {
            type Output=<#wt as #tr_into>::Output;
            fn into_matrix_vector_product(self, rhs:&Rhs) -> <#wt as #tr_into>::Output {
                self.0
                    .into_matrix_vector_product(rhs)
            }
        }

        impl #generics #tr_try for #ty where #wt : #tr_try, Rhs : matrix_traits::ColVectorView, #where_clause {
            type Output=<#wt as #tr_try>::Output;
            fn try_matrix_vector_product(&self, rhs:&Rhs) -> Result<<#wt as #tr_try>::Output,matrix_traits::VectorConstructError> {
                self.0
                    .try_matrix_vector_product(rhs)
            }
        }

        impl #generics #tr_try_into for #ty where #wt : #tr_try_into, Rhs : matrix_traits::ColVectorView, #where_clause {
            type Output=<#wt as #tr_try_into>::Output;
            fn try_into_matrix_vector_product(self, rhs:&Rhs) -> Result<<#wt as #tr_try_into>::Output,matrix_traits::VectorConstructError> {
                self.0
                    .try_into_matrix_vector_product(rhs)
            }
        }
    }.into()
}

/// requires also to implement TryGeneralizedMatrixProduct
// #[proc_macro_derive(TryMatrixVectorProduct)]
// pub fn try_matrix_vector_product_proc_macro(input: TokenStream) -> TokenStream {
//     let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
//     let (generics, where_clause, [(ty, mut wt)])=
//     preprocess_no_impl_add_gen_types(& mut input, vec!["Rhs"]);
//     assert_eq!(wt.len(),1);
//     let wt=wt.remove(0);
//     quote! {
//     }.into()
// }

#[proc_macro_derive(ClosedMatrixMatrixProduct)]
pub fn closed_matrix_matrix_product_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr =quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let tr_into=quote!{matrix_traits::IntoMatrixMatrixProduct};
    let tr_try_into=quote!{matrix_traits::TryIntoMatrixMatrixProduct};
    let (generics, where_clause, [(ty, wt),(ty1, wt1),(ty2, wt2)])=
    preprocess_no_impl(& mut input);
    // let mul_bound=quote!{<#ty as container_traits::ItemT>::T : std::ops::Mul<<#ty1 as container_traits::ItemT>::T>};
    quote! {
        impl #generics #tr<#ty1> for #ty where #(#wt : #tr<#wt1,Output=#wt2>, #wt2 : matrix_traits::Matrix,)*  #where_clause {
            type Output=#ty2;
            fn matrix_matrix_product(&self, rhs:&#ty1) -> #ty2 {
                #ty2(self.0
                         .matrix_matrix_product(&rhs.0))
            }
        }


        impl #generics #tr_into<#ty1> for #ty where #(#wt : #tr_into<#wt1,Output=#wt2>, #wt2 : matrix_traits::Matrix,)* #where_clause {
            type Output=#ty2;
            fn into_matrix_matrix_product(self, rhs:&#ty1) -> #ty2 {
                #ty2(self.0
                         .into_matrix_matrix_product(&rhs.0))
            }
        }

        impl #generics #tr_try<#ty1> for #ty where #(#wt : #tr_try<#wt1,Output=#wt2>, #wt2 : matrix_traits::Matrix,)* #where_clause {
            type Output=#ty2;
            fn try_matrix_matrix_product(&self, rhs:&#ty1) -> Result<#ty2,matrix_traits::MatrixConstructError> {
                self.0
                    .try_matrix_matrix_product(&rhs.0)
                    .map(|c|#ty2(c))
            }
        }

        impl #generics #tr_try_into<#ty1> for #ty where #(#wt : #tr_try_into<#wt1,Output=#wt2>, #wt2 : matrix_traits::Matrix,)* #where_clause {
            type Output=#ty2;
            fn try_into_matrix_matrix_product(self, rhs:&#ty1) -> Result<#ty2,matrix_traits::MatrixConstructError> {
                self.0
                    .try_into_matrix_matrix_product(&rhs.0)
                    .map(|c|#ty2(c))
            }
        }
    }.into()
}



fn matrix_matrix_product_impl(
    input : DeriveInput,
    rhs   : &str,
    omtr  : Option<proc_macro2::TokenStream>,
    ottr  : Option<proc_macro2::TokenStream>) -> TokenStream {
    let rhs=proc_macro2::Ident::new(rhs,proc_macro2::Span::call_site());
    let rhs = quote!(crate::#rhs);
    let tr =parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let tr_into=quote!{matrix_traits::IntoMatrixMatrixProduct};
    let tr_try_into=quote!{matrix_traits::TryIntoMatrixMatrixProduct};
    let hom=quote!{#rhs<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let mut input=input;
    let (generics, wc, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let mut where_clause=quote!{Rhs : matrix_traits::MatrixView};
    if let Some(mtr)=omtr {
        where_clause=quote!{#where_clause, #mtr};
    }
    if let Some(ttr)=ottr {
        where_clause=quote!(#where_clause, Rhs::T : #ttr);
    }
    if !wc.is_empty() {
        where_clause=if where_clause.is_empty() {
          quote!{#wc}
        } else {
          quote!{#where_clause, #wc}
        };
    }
    let wt=types.remove(0);
    quote! {
        impl #generics #tr<#hom> for #ty where #wt : #tr<Rhs>, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(&self, rhs:&#hom) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#hom as container_traits::Inner>::inner(rhs))
            }
        }

        impl #generics #tr_into<#hom> for #ty where #wt : #tr_into<Rhs>, #where_clause {
            type Output=<#wt as #tr_into<Rhs>>::Output;
            fn into_matrix_matrix_product(self, rhs:&#hom) -> <#wt as #tr_into<Rhs>>::Output {
                self.0
                    .into_matrix_matrix_product(<#hom as container_traits::Inner>::inner(rhs))
            }
        }

        impl #generics #tr_try<#hom> for #ty where #wt : #tr_try<Rhs>, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(&self, rhs:&#hom) -> Result<<#wt as #tr_try<Rhs>>::Output,matrix_traits::MatrixConstructError> {
                self.0.try_matrix_matrix_product(<#hom as container_traits::Inner>::inner(rhs))
            }
        }

        impl #generics #tr_try_into<#hom> for #ty where #wt : #tr_try_into<Rhs>, #where_clause {
            type Output=<#wt as #tr_try_into<Rhs>>::Output;
            fn try_into_matrix_matrix_product(self, rhs:&#hom) -> Result<<#wt as #tr_try_into<Rhs>>::Output,matrix_traits::MatrixConstructError> {
                self.0.try_into_matrix_matrix_product(<#hom as container_traits::Inner>::inner(rhs))
            }
        }
    }.into()
    // {
    //     use std::io::Write;
    //     let mut file = std::fs::File::create("matrix_matrix_product_error_".to_string()+&input.ident.to_string()+".txt").unwrap();
    //     writeln!(file, "{res}").unwrap();
    // }
    // res
}


#[proc_macro_derive(MatrixMatrixProductHomogeneous)]
pub fn matrix_matrix_product_hom_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "Homogeneous",
        Some(quote!{matrix_traits::MatrixViewSquare}),
        Some(quote!{algebra_traits::RealNumber}))
}

#[proc_macro_derive(MatrixMatrixProductOrthogonal)]
pub fn matrix_matrix_product_orth_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "Orthogonal",
        Some(quote!{matrix_traits::MatrixViewSquare}),
        Some(quote!{algebra_traits::RealNumber}))
}

#[proc_macro_derive(MatrixMatrixProductUnitary)]
pub fn matrix_matrix_product_unitary_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "Unitary",
        Some(quote!{matrix_traits::MatrixViewSquare}),
        Some(quote!{algebra_traits::ComplexNumber}))
}

#[proc_macro_derive(MatrixMatrixProductSpecialOrthogonal)]
pub fn matrix_matrix_product_special_orth_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "SpecialOrthogonal",
        Some(quote!{matrix_traits::MatrixViewSquare}),
        Some(quote!{algebra_traits::RealNumber}))
}

#[proc_macro_derive(MatrixMatrixProductSpecialUnitary)]
pub fn matrix_matrix_product_special_unitary_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "SpecialUnitary",
        Some(quote!{matrix_traits::MatrixViewSquare}),
        Some(quote!{algebra_traits::ComplexNumber}))
}

#[proc_macro_derive(MatrixMatrixProductStiefel)]
pub fn matrix_matrix_product_stiefel_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "Stiefel",
        None,
        Some(quote!{algebra_traits::Scalar}))
}

#[proc_macro_derive(MatrixMatrixProductNotTall)]
pub fn matrix_matrix_product_not_tall_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "NotTall",
        None,
        None)
}

#[proc_macro_derive(MatrixMatrixProductNotWide)]
pub fn matrix_matrix_product_not_wide_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "NotWide",
        None,
        None)
}

#[proc_macro_derive(MatrixMatrixProductSquare)]
pub fn matrix_matrix_product_square_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "Square",
        None,
        None)
}

#[proc_macro_derive(MatrixMatrixProductTall)]
pub fn matrix_matrix_product_tall_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "Tall",
        None,
        None)
}

#[proc_macro_derive(MatrixMatrixProductWide)]
pub fn matrix_matrix_product_wide_proc_macro(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "Wide",
        None,
        None)
}


#[proc_macro_derive(MatrixMatrixProductAntiHermitian)]
pub fn matrix_matrix_product_anti_herm_proc_macro(input: TokenStream) -> TokenStream {
     let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "AntiHermitian",
        Some(quote!{matrix_traits::MatrixViewSquare}),
        Some(quote!(algebra_traits::ComplexNumber)))
}

#[proc_macro_derive(MatrixMatrixProductHermitian)]
pub fn matrix_matrix_product_herm_proc_macro(input: TokenStream) -> TokenStream {
     let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "Hermitian",
        Some(quote!{matrix_traits::MatrixViewSquare}),
        Some(quote!(algebra_traits::ComplexNumber)))
}

#[proc_macro_derive(MatrixMatrixProductSymmetric)]
pub fn matrix_matrix_product_symm_proc_macro(input: TokenStream) -> TokenStream {
     let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "Symmetric",
        Some(quote!{matrix_traits::MatrixViewSquare}),
        Some(quote!(algebra_traits::RealNumber)))
}

#[proc_macro_derive(MatrixMatrixProductSkewSymmetric)]
pub fn matrix_matrix_product_skew_symm_proc_macro(input: TokenStream) -> TokenStream {
     let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "SkewSymmetric",
        Some(quote!{matrix_traits::MatrixViewSquare}),
        Some(quote!(algebra_traits::RealNumber)))
}

#[proc_macro_derive(MatrixMatrixProductLeftTriangular)]
pub fn matrix_matrix_product_left_triang_proc_macro(input: TokenStream) -> TokenStream {
     let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "LeftTriangular",
        None,
        Some(quote!(num_traits::Zero)))
}

#[proc_macro_derive(MatrixMatrixProductRightTriangular)]
pub fn matrix_matrix_product_right_triang_proc_macro(input: TokenStream) -> TokenStream {
     let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    matrix_matrix_product_impl(
        input,
        "RightTriangular",
        None,
        Some(quote!(num_traits::Zero)))
}


#[proc_macro_derive(Identity)]
pub fn identity_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::Identity};
    let (generics, where_clause, [(ty, mut wt)])=
    preprocess_no_impl(& mut input);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            fn identity(n:usize) -> Self {
                Self(<#wt as #tr>::identity(n))
            }
        }
    }.into()
}

#[proc_macro_derive(IntoBaseMatrix)]
pub fn into_base_matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::IntoBaseMatrix};
    let (generics, where_clause, ty, wt)=
    preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            type Output=<#wt as #tr>::Output;
            fn into_base_matrix(self) -> <#wt as #tr>::Output {
                <#wt as #tr>::into_base_matrix(self.0)
            }
        }
    }.into()
}

#[proc_macro_derive(AsBaseMatrix)]
pub fn as_base_matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::AsBaseMatrix};
    let (generics, where_clause, ty, wt)=
    preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            type Output=<#wt as #tr>::Output;
            fn base_matrix(&self) -> &<#wt as #tr>::Output {
                <#wt as #tr>::base_matrix(&self.0)
            }
        }
    }.into()
}

#[proc_macro_derive(IntoBaseSquareMatrix)]
pub fn into_base_square_matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::IntoBaseSquareMatrix};
    let (generics, where_clause, ty, wt)=
    preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            type Output=<#wt as #tr>::Output;
            fn into_base_square_matrix(self) -> <#wt as #tr>::Output {
                <#wt as #tr>::into_base_square_matrix(self.0)
            }
        }
    }.into()
}

#[proc_macro_derive(AsBaseSquareMatrix)]
pub fn as_base_square_matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::AsBaseSquareMatrix};
    let (generics, where_clause, ty, wt)=
    preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            type Output=<#wt as #tr>::Output;
            fn base_square_matrix(&self) -> &<#wt as #tr>::Output {
                <#wt as #tr>::base_square_matrix(&self.0)
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixViewNotWide)]
pub fn matrix_not_wide_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixViewNotWide};
    let (generics, where_clause, ty, wt)=preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixViewNotTall)]
pub fn matrix_not_tall_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixViewNotTall};
    let (generics, where_clause, ty, wt)=preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixViewSquare)]
pub fn matrix_square_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixViewSquare};
    let (generics, where_clause, ty, wt)=preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixViewWide)]
pub fn matrix_wide_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixViewWide};
    let (generics, where_clause, ty, wt)=preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixViewTall)]
pub fn matrix_tall_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixViewTall};
    let (generics, where_clause, ty, wt)=preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixShape)]
pub fn matrix_shape_proc_macro(input: TokenStream) -> TokenStream {
    let fs=[
        matrix_not_tall_proc_macro,
        matrix_not_wide_proc_macro,
        matrix_square_proc_macro,
        matrix_wide_proc_macro,
        matrix_tall_proc_macro
    ];
    fs.into_iter()
      .map(|f|f(input.clone()))
      .collect()
    // for i in 1..10 {
    //     for j in 1..10 {
    //         let string=format!("Matrix{i}{j}");
    //         let ident=syn::Ident::new(&string,proc_macro2::Span::call_site());
    //         v.push(quote!{matrix_traits::matrices::matrix_shapes::#ident });
    //     }
    // }
}

#[proc_macro_derive(MatrixSquareTryConstruct)]
pub fn matrix_square_try_construct_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixSquareTryConstruct};
    let (generics, where_clause, ty, _)=
    preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty
        where Self : matrix_traits::MatrixViewSquare
                    +matrix_traits::MatrixTryConstruct, #where_clause {}
    }.into()
}

#[proc_macro_derive(TryInvFromTryInvCoarse)]
pub fn try_inv_from_try_inv_coarse(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{algebra_traits::TryInv};
    let tr_c=quote!{matrix_traits::TryInvCoarse};
    let (generics, where_clause, ty, _)=
    preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty
        where Self : Clone
                    +algebra_traits::TryInv<Output=Self>
                    +matrix_traits::MatrixViewSquare
                    +matrix_traits::TryMatrixMatrixProduct<Output=Self>
                    +#tr_c<Output=Self>,
            #where_clause {
            type Output=Self;
            type Error=<Self as algebra_traits::TryInv>::Error;
            fn is_invertible(&self) -> Result<(),<Self as algebra_traits::TryInv>::Error>
            {
                self.clone()
                    .try_inv()
                    .map(|_|())
            }
            fn try_inv(self) -> Result<Self,<Self as algebra_traits::TryInv>::Error> {
                let mut si=<Self as #tr_c>::try_inv_coarse(self.clone())?;
                let mul=|lhs:&Self,rhs:&Self|lhs.clone().try_matrix_matrix_product(rhs.clone()).unwrap();
                for _ in 0..5 {
                    si=mul(&si,&<Self as #tr_c>::try_inv_coarse(mul(&self,&si))?);
                }
                Some(si)
            }
        }
    }.into()
}

#[proc_macro_derive(TryIntoSubMatrix)]
pub fn try_into_sub_matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr:syn::Path=parse_quote!{matrix_traits::TryIntoSubMatrix};
    let fn_name=parse_quote!{ try_into_sub_matrix };
    let (generics, where_clause, [(ty, wt),(ty1, wt1)], _)=
    DeriveHelper::new(&mut input, &tr, &fn_name).extended1();
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr<Output=#wt1>, #wt1 : matrix_traits::MatrixTryConstruct,)* #where_clause {
            type Output=#ty1;
            fn try_into_sub_matrix(self,size:(usize,usize)) -> Option<#ty1> {
                self.0
                    .try_into_sub_matrix(size)
                    .map(|s|#ty1(s))
            }
        }
    }.into()
}

#[proc_macro_derive(Display)]
pub fn display_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name=input.ident.to_string();
    let tr=quote!{std::fmt::Display};
    let (generics, where_clause, ty, wt)=
    preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
              writeln!(f,#struct_name)?;
              <#wt as #tr>::fmt(&self.0, f)
            }
        }
    }.into()
}

#[proc_macro_derive(Inherit)]
pub fn inherit_proc_macro(input: TokenStream) -> TokenStream {
    let fs=[
        matrix_view_proc_macro,
        matrix_proc_macro,
        algebra_matrix_proc_macro,
        display_proc_macro,
        as_base_matrix_proc_macro,
        into_base_matrix_proc_macro,
        into_dyn_matrix_proc_macro,
        as_base_square_matrix_proc_macro,
        into_base_square_matrix_proc_macro,
        static_matrix_proc_macro,
        matrix_vector_product_proc_macro,
        matrix_try_construct_proc_macro,
        matrix_square_try_construct_proc_macro
    ];
    fs.into_iter()
      .map(|f|f(input.clone()))
      .collect()
}