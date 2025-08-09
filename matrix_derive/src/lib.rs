
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

#[proc_macro_derive(Matrix)]
pub fn matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, where_clause, ty, wt) = preprocess4matrix(&mut input);
    let tr=quote!{matrix_traits::Matrix};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            type Row=#wt::Row;
            type Col=#wt::Col;
            fn nrows(&self) -> usize {
                <#wt as #tr>::nrows(&self.0)
            }
            fn ncols(&self) -> usize {
                <#wt as #tr>::ncols(&self.0)
            }
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
            fn try_col_sc_prod(&self, j0:usize, j1:usize) -> Option<Self::T> where Self::T:Clone
            {
                <#wt as #tr>::try_col_sc_prod(&self.0,j0,j1)
            }
        }
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
    let fn_name=parse_quote!{ transpose };
    let (generics, where_clause, [(ty, wt),(ty1, wt1)], implementation)=
    DeriveHelper::new(&mut input, &tr, &fn_name).extended1();
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr<Output=#wt1>,)* #where_clause {
            type Output=#ty1;
            fn transpose(self) -> #ty1 {
                #implementation
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
            fn transpose(self) -> Self {
                Self(self.0
                         .transpose())
            }
        }
    }.into()
}

#[proc_macro_derive(Empty)]
pub fn empty_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr = quote!{container_traits::Empty};
    let (generics, where_clause, ty, wt) =  preprocess4matrix(&mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            fn empty() -> Self {
                Self(<#wt>::empty())
            }

            fn is_empty(&self) -> bool {
                <Self as matrix_traits::Matrix>::matrix_dimensions(&self) == (0,0)
            }
        }
    }.into()
}

#[proc_macro_derive(StaticMatrix)]
pub fn static_matrix_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr = quote!{matrix_traits::StaticMatrix};
    let tr_sqr=quote!{matrix_traits::SquareStaticMatrix};
    let (generics, where_clause, ty, wt) =  preprocess4matrix(&mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {
            const M:usize=<#wt as #tr>::M;
            const N:usize=<#wt as #tr>::N;
        }

        impl #generics #tr_sqr for #ty where #wt : #tr_sqr, Self : matrix_traits::MatrixSquare, #where_clause {
            const M:usize=<#wt as #tr_sqr>::M;
        }
    }.into()
}

/// requires also to implement GeneralizedMatrixProduct
#[proc_macro_derive(MatrixVectorProduct)]
pub fn matrix_vector_product_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr = quote!{matrix_traits::MatrixVectorProduct<Rhs>};
    let tr_try = quote!{matrix_traits::TryMatrixVectorProduct<Rhs>};
    let (generics, where_clause, [(ty, mut wt)])=
    preprocess_no_impl_add_gen_types(& mut input, vec!["Rhs"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, Rhs : matrix_traits::ColVector, #where_clause {
            type Output=<#wt as #tr>::Output;
            fn matrix_vector_product(self, rhs:Rhs) -> <#wt as #tr>::Output {
                self.0
                    .matrix_vector_product(rhs)
            }
        }

        impl #generics #tr_try for #ty where #wt : #tr_try, Rhs : matrix_traits::ColVector, #where_clause {
            type Output=<#wt as #tr_try>::Output;
            fn try_matrix_vector_product(self, rhs:Rhs) -> Option<<#wt as #tr_try>::Output> {
                self.0
                    .try_matrix_vector_product(rhs)
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
    let (generics, where_clause, [(ty, wt),(ty1, wt1),(ty2, wt2)])=
    preprocess_no_impl(& mut input);
    let mul_bound=quote!{<#ty as container_traits::ItemT>::T : std::ops::Mul<<#ty1 as container_traits::ItemT>::T>};
    quote! {
        impl #generics #tr<#ty1> for #ty where #(#wt : #tr<#wt1,Output=#wt2>,)* #mul_bound, #where_clause {
            type Output=#ty2;
            fn matrix_matrix_product(self, rhs:#ty1) -> #ty2 {
                #ty2(self.0
                         .matrix_matrix_product(rhs.0))
            }
        }

        impl #generics #tr_try<#ty1> for #ty where #(#wt : #tr_try<#wt1,Output=#wt2>,)* #mul_bound, #where_clause {
            type Output=#ty2;
            fn try_matrix_matrix_product(self, rhs:#ty1) -> Option<#ty2> {
                self.0
                    .try_matrix_matrix_product(rhs.0)
                    .map(|c|#ty2(c))
            }
        }
    }.into()
}




#[proc_macro_derive(MatrixMatrixProductHomogeneous)]
pub fn matrix_matrix_product_hom_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let hom=quote!{Homogeneous<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#hom> for #ty where #wt : #tr<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::RealNumber, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#hom) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#hom as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#hom> for #ty where #wt : #tr_try<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::RealNumber, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#hom) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#hom as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductOrthogonal)]
pub fn matrix_matrix_product_orth_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let orth=quote!{Orthogonal<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#orth> for #ty where #wt : #tr<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::RealNumber, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#orth) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#orth as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#orth> for #ty where #wt : #tr_try<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::RealNumber, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#orth) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#orth as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductUnitary)]
pub fn matrix_matrix_product_unitary_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let unitary=quote!{Unitary<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#unitary> for #ty where #wt : #tr<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::ComplexNumber, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#unitary) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#unitary as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#unitary> for #ty where #wt : #tr_try<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::ComplexNumber, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#unitary) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#unitary as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductSpecialOrthogonal)]
pub fn matrix_matrix_product_special_orth_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let orth=quote!{Orthogonal<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#orth> for #ty where #wt : #tr<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::RealNumber, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#orth) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#orth as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#orth> for #ty where #wt : #tr_try<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::RealNumber, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#orth) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#orth as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductSpecialUnitary)]
pub fn matrix_matrix_product_special_unitary_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let unitary=quote!{SpecialUnitary<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#unitary> for #ty where #wt : #tr<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::ComplexNumber, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#unitary) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#unitary as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#unitary> for #ty where #wt : #tr_try<Rhs>, Rhs : matrix_traits::MatrixSquare, Rhs::T : algebra_traits::ComplexNumber, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#unitary) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#unitary as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductStiefel)]
pub fn matrix_matrix_product_stiefel_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let stiefel=quote!{Stiefel<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#stiefel> for #ty where #wt : #tr<Rhs>, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#stiefel) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#stiefel as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#stiefel> for #ty where #wt : #tr_try<Rhs>, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#stiefel) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#stiefel as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductNotTall)]
pub fn matrix_matrix_product_not_tall_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let not_tall=quote!{NotTall<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#not_tall> for #ty where #wt : #tr<Rhs>, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#not_tall) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#not_tall as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#not_tall> for #ty where #wt : #tr_try<Rhs>, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#not_tall) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#not_tall as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductNotWide)]
pub fn matrix_matrix_product_not_wide_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let not_wide=quote!{NotWide<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#not_wide> for #ty where #wt : #tr<Rhs>, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#not_wide) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#not_wide as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#not_wide> for #ty where #wt : #tr_try<Rhs>, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#not_wide) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#not_wide as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductSquare)]
pub fn matrix_matrix_product_square_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let square=quote!{Square<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#square> for #ty where #wt : #tr<Rhs>, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#square) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#square as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#square> for #ty where #wt : #tr_try<Rhs>, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#square) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#square as container_traits::IntoInner>::into_inner(rhs))
            }
        }<Rhs>
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductTall)]
pub fn matrix_matrix_product_tall_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let tall=quote!{Tall<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#tall> for #ty where #wt : #tr<Rhs>, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#tall) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#tall as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#tall> for #ty where #wt : #tr_try<Rhs>, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#tall) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#tall as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductWide)]
pub fn matrix_matrix_product_wide_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let wide=quote!{Wide<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#wide> for #ty where #wt : #tr<Rhs>, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#wide) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#wide as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#wide> for #ty where #wt : #tr_try<Rhs>, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#wide) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#wide as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}


#[proc_macro_derive(MatrixMatrixProductAntiHermitian)]
pub fn matrix_matrix_product_anti_herm_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let anti_herm=quote!{AntiHermitian<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#anti_herm> for #ty where
        #wt : #tr<Rhs>,
        Rhs : matrix_traits::MatrixSquare,
        Rhs :: T : Clone+algebra_traits::ComplexNumber, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#anti_herm) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#anti_herm as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#anti_herm> for #ty where
        #wt : #tr_try<Rhs>,
        Rhs : matrix_traits::MatrixSquare,
        Rhs :: T : Clone+algebra_traits::ComplexNumber, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#anti_herm) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#anti_herm as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductHermitian)]
pub fn matrix_matrix_product_herm_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let herm=quote!{Hermitian<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#herm> for #ty where
        #wt : #tr<Rhs>,
        Rhs : matrix_traits::MatrixSquare,
        Rhs :: T : Clone+algebra_traits::ComplexNumber, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#herm) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#herm as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#herm> for #ty where
        #wt : #tr_try<Rhs>,
        Rhs : matrix_traits::MatrixSquare,
        Rhs :: T : Clone+algebra_traits::ComplexNumber,  #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#herm) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#herm as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductSymmetric)]
pub fn matrix_matrix_product_symm_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let symm=quote!{Symmetric<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#symm> for #ty where 
            #wt : #tr<Rhs>,
            Rhs :: T : Clone+algebra_traits::RealNumber,
            Rhs : matrix_traits::MatrixSquare, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#symm) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#symm as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#symm> for #ty where
            #wt : #tr_try<Rhs>,
            Rhs :: T : Clone+algebra_traits::RealNumber,
            Rhs : matrix_traits::MatrixSquare, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#symm) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#symm as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductSkewSymmetric)]
pub fn matrix_matrix_product_skew_symm_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let skew_symm=quote!{SkewSymmetric<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#skew_symm> for #ty where 
            #wt : #tr<Rhs>,
            Rhs :: T : Clone+algebra_traits::RealNumber,
            Rhs : matrix_traits::MatrixSquare, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#skew_symm) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#skew_symm as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#skew_symm> for #ty where
            #wt : #tr_try<Rhs> + matrix_traits::MatrixSquare,
            Rhs :: T : Clone+algebra_traits::RealNumber,
            Rhs : matrix_traits::MatrixSquare, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#skew_symm) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#skew_symm as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductLeftTriangular)]
pub fn matrix_matrix_product_left_triang_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let left_triang=quote!{LeftTriangular<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#left_triang> for #ty where
            #wt : #tr<Rhs>,
            Rhs :: T : num_traits::Zero,
            Rhs :  matrix_traits::Matrix, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#left_triang) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#left_triang as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#left_triang> for #ty where
            #wt : #tr_try<Rhs>,
            Rhs :: T : num_traits::Zero,
            Rhs :  matrix_traits::Matrix, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#left_triang) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#left_triang as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
}

#[proc_macro_derive(MatrixMatrixProductRightTriangular)]
pub fn matrix_matrix_product_right_triang_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{matrix_traits::MatrixMatrixProduct};
    let tr_try=quote!{matrix_traits::TryMatrixMatrixProduct};
    let right_triang=quote!{RightTriangular<Rhs>};
    let fn_name=parse_quote!{matrix_matrix_product};
    let (generics, where_clause, [(ty, mut types)], _)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["Rhs"]).binary_const_rhs(false,&parse_quote!{rhs});
    let wt=types.remove(0);

    quote! {
        impl #generics #tr<#right_triang> for #ty where
            #wt : #tr<Rhs>,
            Rhs :: T : num_traits::Zero,
            Rhs :  matrix_traits::Matrix, #where_clause {
            type Output=<#wt as #tr<Rhs>>::Output;
            fn matrix_matrix_product(self, rhs:#right_triang) -> <#wt as #tr<Rhs>>::Output {
                self.0.matrix_matrix_product(<#right_triang as container_traits::IntoInner>::into_inner(rhs))
            }
        }

        impl #generics #tr_try<#right_triang> for #ty where
            #wt : #tr_try<Rhs>,
            Rhs :: T : num_traits::Zero,
            Rhs :  matrix_traits::Matrix, #where_clause {
            type Output=<#wt as #tr_try<Rhs>>::Output;
            fn try_matrix_matrix_product(self, rhs:#right_triang) -> Option<<#wt as #tr_try<Rhs>>::Output> {
                self.0.try_matrix_matrix_product(<#right_triang as container_traits::IntoInner>::into_inner(rhs))
            }
        }
    }.into()
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

#[proc_macro_derive(MatrixNotWide)]
pub fn matrix_not_wide_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixNotWide};
    let (generics, where_clause, ty, wt)=preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixNotTall)]
pub fn matrix_not_tall_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixNotTall};
    let (generics, where_clause, ty, wt)=preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixSquare)]
pub fn matrix_square_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixSquare};
    let (generics, where_clause, ty, wt)=preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixWide)]
pub fn matrix_wide_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixWide};
    let (generics, where_clause, ty, wt)=preprocess4matrix(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #where_clause {}
    }.into()
}

#[proc_macro_derive(MatrixTall)]
pub fn matrix_tall_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{matrix_traits::MatrixTall};
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
        where Self : matrix_traits::MatrixSquare
                    +matrix_traits::MatrixTryConstruct, #where_clause {}
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
        empty_proc_macro,
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