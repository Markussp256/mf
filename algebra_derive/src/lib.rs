//! # algebra_derive
//!
//! This crate provides procedural macros to implement functions and/or traits for a wrapper.
//! 

use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, DeriveInput, Path};
use quote::quote;


use derive_helper::{preprocessor::*, rhs_subfields, self_subfields, fields_trait::Fields, Arity, DeriveHelper};

/// Implements [`algebra_traits::Conjugate`]
///
/// complex conjugation
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::Conjugate;
/// use algebra_traits::Conjugate;
/// use num::complex::Complex;
///
/// #[derive(Conjugate, PartialEq, Debug)]
/// struct MyWrapper<C>(C);
/// 
/// let w=MyWrapper(Complex{re:1.0, im:2.0});
/// let con_w=w.conjugate();
///
/// assert_eq!(con_w, MyWrapper(Complex{re:1.0, im:-2.0}));
/// ```
#[proc_macro_derive(Conjugate)]
pub fn conjugate_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::Conjugate};
    let fn_name=parse_quote!{ conjugate };
    let (generics, wc, [(ty, types)], implementation)=
    DeriveHelper::new(& mut input, &tr, &fn_name).preprocess(Arity::Unary);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr,)* #wc  {
            fn conjugate(self) ->  #ty {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::ConstElement`]
///
/// complex conjugation
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::ConstElement;
/// use algebra_traits::ConstElement;
///
/// #[derive(ConstElement, PartialEq, Debug)]
/// struct MyWrapper<C>(C);
///
/// assert_eq!(MyWrapper::<f64>::ELEMENT, MyWrapper(f64::ELEMENT));
/// ```
#[proc_macro_derive(ConstElement)]
pub fn const_element_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name=input.ident.clone();
    let tr=quote!{algebra_traits::ConstElement};
    let (generics, wc, [(ty, wt)])=preprocess_no_impl(& mut input);
    let exprs:Vec<syn::Expr>=
        wt.iter()
          .map(|t|parse_quote!{<#t as #tr>::ELEMENT})
          .collect();
    let prep=Preprocessor::new(& mut input);
    let fields=prep.fields();
    let implementation=fields.struct_literal(&struct_name, exprs);
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr,)* #wc  {
            const ELEMENT:#ty=#implementation;
        }
    }.into()
}

/// Implements [`algebra_traits::ConstNonZero`]
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::ConstNonZero;
/// use algebra_traits::ConstNonZero;
/// 
/// #[derive(Debug,ConstNonZero,PartialEq)]
/// struct Myf64(f64);
///
/// assert!(<Myf64 as ConstNonZero>::NONZERO != Myf64(0.0));
/// ```
#[proc_macro_derive(ConstNonZero)]
pub fn const_nonzero_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::ConstNonZero};
    let fn_name=parse_quote!{ NONZERO };
    let (generics, wc, [(ty, types)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).nullary_const();
    quote! {
        impl #generics #tr for #ty where #(#types : #tr,)* #wc  {
            const NONZERO:Self=#implementation;
        }
    }.into()
}

/// Implements [`std::ops::Add`]
/// 
/// also contains ClosedAdd
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::Add;
/// 
/// #[derive(PartialEq, Debug)]
/// struct Time(f64);
/// 
/// struct Duration(f64);
/// 
/// impl std::ops::Add<Duration> for Time {
///     type Output=Self;
///     fn add(self, v:Duration) -> Self {
///         Self(self.0+v.0)
///     }
/// }
/// 
/// #[derive(Add, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let p=MyWrapper(Time(2.0));
/// let v=MyWrapper(Duration(3.0));
/// assert_eq!(p+v, MyWrapper(Time(5.0)));
/// ```
#[proc_macro_derive(Add)]
pub fn add_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{std::ops::Add};
    let fn_name=parse_quote!{add};
    let (generics, wc, [(ty, types),(ty1, types1),(ty2, types2)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).extended2();
    quote! {
        impl #generics #tr<#ty1> for #ty where #(#types : #tr<#types1,Output=#types2>,)* #wc {
            type Output=#ty2;
            fn add(self, rhs : #ty1) -> #ty2 {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::ops::Mul`]
/// 
/// also contains ClosedMul
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::Mul;
/// 
/// #[derive(Mul, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// ```
#[proc_macro_derive(Mul)]
pub fn mul_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{std::ops::Mul};
    let fn_name=parse_quote!{mul};
    let (generics, wc, [(ty, types),(ty1, types1),(ty2, types2)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).extended2();
    quote! {
        impl #generics #tr<#ty1> for #ty where #(#types : #tr<#types1,Output=#types2>,)* #wc {
            type Output=#ty2;
            fn mul(self, rhs : #ty1) -> #ty2 {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::ops::Add`]  with Output=Self
/// 
/// addition with itself
/// 
/// note : we can not simultaneously use derive_macro Add and ClosedAdd,
/// Add is a generalization of ClosedAdd
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::ClosedAdd;
/// 
/// #[derive(ClosedAdd, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let a=MyWrapper(2_u32);
/// let b=MyWrapper(3_u32);
/// assert_eq!(a+b, MyWrapper(5_u32));
/// ```
#[proc_macro_derive(ClosedAdd)]
pub fn closed_add_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{std::ops::Add};
    let fn_name=parse_quote!{add};
    let (generics, wc, [(ty, types)], implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).preprocess(Arity::Binary);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types>,)* #wc {
            type Output=Self;
            fn add(self, rhs : Self) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::ops::AddAssign`]  with Rhs=Self
/// using *self=self.clone()+rhs
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::{ClosedAdd,AddAssignFromAdd};
/// 
/// #[derive(ClosedAdd, AddAssignFromAdd, Clone, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let mut a=MyWrapper(2_u32);
/// let b=MyWrapper(3_u32);
/// a+=b;
/// assert_eq!(a, MyWrapper(5_u32));
/// ```
#[proc_macro_derive(AddAssignFromAdd)]
pub fn add_assign_from_add_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, _)])=
    preprocess_no_impl(& mut input);
    quote! {
        impl #generics std::ops::AddAssign for #ty where Self : Clone+std::ops::Add<Output=Self>, #wc {
            fn add_assign(&mut self, rhs: Self) {
                *self=self.clone()+rhs
            }
        }
    }.into()
}

/// Implements [`std::ops::SubAssign`]  with Rhs=Self
/// using *self=self.clone()-rhs
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::{ClosedSub,SubAssignFromSub};
/// 
/// #[derive(ClosedSub, SubAssignFromSub, Clone, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let mut a=MyWrapper(3_u32);
/// let b=MyWrapper(2_u32);
/// a-=b;
/// assert_eq!(a, MyWrapper(1_u32));
/// ```
#[proc_macro_derive(SubAssignFromSub)]
pub fn sub_assign_from_sub_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, _)])=
    preprocess_no_impl(& mut input);
    quote! {
        impl #generics std::ops::SubAssign for #ty where Self : Clone + std::ops::Sub<Output=Self>, #wc {
            fn sub_assign(&mut self, rhs: Self) {
                *self=self.clone()-rhs
            }
        }
    }.into()
}

/// Implements [`std::ops::Mul`]  with Output=Self
/// 
/// multiplication with itself
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::ClosedMul;
/// 
/// #[derive(ClosedMul, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let a=MyWrapper(2_u32);
/// let b=MyWrapper(3_u32);
/// assert_eq!(a*b, MyWrapper(6_u32));
/// ```
#[proc_macro_derive(ClosedMul)]
pub fn closed_mul_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{std::ops::Mul};
    let fn_name=parse_quote!{mul};
    let (generics, wc, [(ty, types)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).preprocess(Arity::Binary);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types>,)* #wc {
            type Output=Self;
            fn mul(self, rhs : Self) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::ops::Div`]  with Output=Self
/// 
/// division with itself
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::ClosedDiv;
/// 
/// #[derive(ClosedDiv, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let a=MyWrapper(3.0);
/// let b=MyWrapper(2.0);
/// assert_eq!(a/b, MyWrapper(1.5));
/// ```
#[proc_macro_derive(ClosedDiv)]
pub fn closed_div_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{std::ops::Div};
    let fn_name=parse_quote!{div};
    let (generics, wc, [(ty, types)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).preprocess(Arity::Binary);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types>,)* #wc {
            type Output=Self;
            fn div(self, rhs : Self) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`num_traits::Zero`]
/// 
/// requires to implement Add
/// use ClosedAdd or Add
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::{Zero,ClosedAdd};
/// use num_traits::Zero; 
///
/// #[derive(Debug, PartialEq, Zero, ClosedAdd)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper::<i32>::zero();
/// assert_eq!(w, MyWrapper(0));
/// 
/// assert!(w.is_zero());
///
/// let w:MyWrapper::<i32>=MyWrapper(1);
/// assert_eq!(w.is_zero(), false);
/// ```
#[proc_macro_derive(Zero)]
pub fn zero_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{num_traits::Zero};
    let fn_name=parse_quote!{zero};
    let (generics, wc, [(ty, types)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).preprocess(Arity::Nullary);
    let subfields=self_subfields(& mut input);
    quote! {
       impl #generics #tr for #ty
        where Self : std::ops::Add<Output=Self>,  #(#types : #tr,)* #wc  {
            fn zero() -> Self {
                #implementation
            }

            fn is_zero(&self) -> bool {
                #(<#types as #tr>::is_zero(& #subfields ) && )* true
            }
        }
    }.into()
}

/// Implements [`num_traits::One`]
/// 
/// requires to implement Mul
/// use ClosedMul or Mul
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::{One,ClosedMul};
/// use num_traits::One; 
///
/// #[derive(Debug, PartialEq, One, ClosedMul)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper::<f64>::one();
/// assert_eq!(w, MyWrapper(1.0));
/// ```
#[proc_macro_derive(One)]
pub fn one_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{num_traits::One};
    let fn_name=parse_quote!{one};
    let (generics, wc, [(ty, types)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).preprocess(Arity::Nullary);
    quote! {
       impl #generics #tr for #ty
        where Self : std::ops::Mul<Output=Self>, #(#types : #tr,)* #wc {
            fn one() -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::ops::Sub`] with Output=Self
/// 
/// subtraction with itself
/// 
/// note : we can not simultaneously use derive_macro Sub and ClosedSub,
/// Sub is a generalization of ClosedSub
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::ClosedSub;
/// 
/// #[derive(ClosedSub, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let a:MyWrapper<i32>=MyWrapper(3);
/// let b:MyWrapper<i32>=MyWrapper(2);
/// let c:MyWrapper<i32>=MyWrapper(1);
/// assert_eq!(a-b, c);
/// ```
#[proc_macro_derive(ClosedSub)]
pub fn closed_sub_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{std::ops::Sub};
    let fn_name=parse_quote!{sub};
    let (generics, wc, [(ty, types)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).preprocess(Arity::Binary);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types>,)* #wc {
            type Output=Self;
            fn sub(self, rhs : Self) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::ops::Sub`] whenever the wrapped type satisfies it
/// 
/// this also contains ClosedSub
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::Sub;
///
/// #[derive(Sub, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let w0=MyWrapper(3);
/// let w1=MyWrapper(2);
/// let wd=w0-w1;
/// assert_eq!(wd, MyWrapper(1));
/// ```
#[proc_macro_derive(Sub)]
pub fn sub_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{std::ops::Sub};
    let fn_name=parse_quote!{sub};
    let (generics, wc, [(ty, types),(ty1, types1),(ty2, types2)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).extended2();
    quote! {
        impl #generics #tr<#ty1> for #ty where #(#types : #tr<#types1,Output=#types2>,)* #wc {
            type Output=#ty2;
            fn sub(self, rhs : #ty1) -> #ty2 {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::Distance`] whenever the wrapped type satisfies it
/// 
/// # Example
/// 
/// ```rust
/// use algebra_derive::Distance;
/// use algebra_traits::{Distance, Nonnegative};
///
/// #[derive(Distance, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let w0=MyWrapper(3.0);
/// let w1=MyWrapper(2.0);
/// let wd=w0.distance(w1);
/// assert_eq!(wd.into_signed(), 1.0);
/// ```
#[proc_macro_derive(Distance)]
pub fn distance_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let try_tr=quote!{algebra_traits::TryDistance};
    let tr=    quote!{algebra_traits::Distance};
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl(&mut input);
    assert_eq!(wt.len(),1);
    let wt=&wt[0];
    quote! {
        impl #generics #try_tr for #ty where #wt : #try_tr, #wc {
            type TryDistT=<#wt as #try_tr>::TryDistT;
            type Error=<#wt as #try_tr>::Error;
            fn try_distance(self, rhs : impl Into<Self>) -> Result<algebra_traits::Nonnegative<Self::TryDistT>,
                                                                   <#wt as #try_tr>::Error> {
                let rhs:Self=rhs.into();
                self.0
                    .try_distance(rhs.0)
            }
        }

        impl #generics #tr for #ty where #wt : #tr, #wc {
            type DistT=<#wt as #tr>::DistT;
            fn distance(self, rhs : impl Into<Self>) -> algebra_traits::Nonnegative<<#wt as #tr>::DistT> {
                let rhs:Self=rhs.into();
                self.0
                    .distance(rhs.0)
            }
        }
    }.into()
}

/// Implements [`std::ops::Neg`]
/// 
/// unary negation
/// # Example
/// 
/// ```rust
/// use algebra_derive::Neg;
///
/// struct Negative(u32);
/// 
/// impl std::ops::Neg for Negative {
///     type Output=u32;
///     fn neg(self) -> u32 {
///         self.0
///     }
/// }
///  
/// #[derive(Neg, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper(Negative(42_u32));
/// assert_eq!(-w, MyWrapper(42_u32));
/// ```
#[proc_macro_derive(Neg)]
pub fn neg_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{std::ops::Neg};
    let fn_name=parse_quote!{neg};
    let (generics, wc, [(ty, types),(ty1, types1)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).extended1();
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types1>,)* #wc  {
            type Output=#ty1;
            fn neg(self) -> #ty1 {
                #implementation
            }
        }
    }.into()
}

/// Implements [`num_traits::Inv`]
/// 
/// inversion
/// # Example
/// 
/// ```rust
/// use algebra_derive::Inv;
/// use num_traits::Inv;
/// 
/// #[derive(Inv, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// ```
#[proc_macro_derive(Inv)]
pub fn inv_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{num_traits::Inv};
    let fn_name=parse_quote!{inv};
    let (generics, wc, [(ty, types),(ty1, types1)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).extended1();
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types1>,)* #wc {
            type Output=#ty1;
            fn inv(self) -> #ty1 {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::ops::Neg`] with Output=Self
/// 
/// closed unary negation
/// # Example
/// 
/// ```rust
/// use algebra_derive::ClosedNeg;
///  
/// #[derive(ClosedNeg, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper(42_i32);
/// assert_eq!(-w, MyWrapper(-42_i32));
/// ```
#[proc_macro_derive(ClosedNeg)]
pub fn closed_neg_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{std::ops::Neg};
    let fn_name=parse_quote!{neg};
    let (generics, wc, [(ty, types)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).preprocess(Arity::Unary);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types>,)* #wc {
            type Output=Self;
            fn neg(self) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`num_traits::Inv`] with Output=Self
/// 
/// closed unary inversion
/// # Example
/// 
/// ```rust
/// use algebra_derive::ClosedInv;
/// use num_traits::Inv; 
/// 
/// #[derive(ClosedInv, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper(42_f64);
/// assert_eq!(w.inv(), MyWrapper(1.0/42_f64));
/// ```
#[proc_macro_derive(ClosedInv)]
pub fn closed_inv_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{num_traits::Inv};
    let fn_name=parse_quote!{inv};
    let (generics, wc, [(ty, types)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).preprocess(Arity::Unary);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types>,)* #wc {
            type Output=Self;
            fn inv(self) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::ScalarMul`] and std::ops::Mul<f64>
/// 
/// scalar multiplication
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::ScalarMul;
/// use algebra_traits::ScalarMul;
/// 
/// #[derive(ScalarMul, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper([0.0, 1.0, 2.0, 3.0]);
/// let wm=w.scalar_mul(&3.0);
/// assert_eq!(wm,MyWrapper([0.0, 3.0, 6.0, 9.0]));
/// ```
#[proc_macro_derive(ScalarMul)]
pub fn scalar_mul_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::ScalarMul<FScalarMul>};
    let fn_name=parse_quote!{scalar_mul};
    let (generics, wc, [(ty, types)],implementation)=
        DeriveHelper::new(& mut input,&tr,&fn_name)
            .add_gen_types(vec!["FScalarMul"])
            .binary_const_rhs(false,&parse_quote!{f});
    quote! {
        impl #generics #tr for #ty where #(#types : #tr,)* #wc  {
            fn scalar_mul(self, f:&FScalarMul) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::Scalarproduct`]
/// 
/// scalar product
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::Scalarproduct;
/// use algebra_traits::{Scalarproduct,TryScalarproduct};
/// 
/// #[derive(Scalarproduct, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let a=MyWrapper([0.0, 1.0, 2.0]);
/// let b=MyWrapper([1.0, 2.0, 3.0]);
/// assert_eq!(a.scalar_product(b),8.0);
/// 
/// let a=MyWrapper([0.0, 1.0, 2.0]);
/// let b=MyWrapper([1.0, 2.0, 3.0]);
/// assert_eq!(a.try_scalar_product(b),Some(8.0));
/// 
/// let a=MyWrapper(vec![0.0, 1.0, 2.0]);
/// let b=MyWrapper(vec![1.0, 2.0, 3.0]);
/// assert_eq!(a.try_scalar_product(b),Some(8.0));
/// 
/// let a=MyWrapper(vec![0.0, 1.0]);
/// let b=MyWrapper(vec![1.0, 2.0, 3.0]);
/// assert!(a.try_scalar_product(b).is_none());
/// ```
#[proc_macro_derive(Scalarproduct)]
pub fn scalar_product_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr:Path    =parse_quote!{algebra_traits::Scalarproduct};
    let tr_try:Path=parse_quote!{algebra_traits::TryScalarproduct};
    let (generics, wc, [(ty, types)])=
        preprocess_no_impl_add_gen_types(& mut input, vec!["ScProdT"]);
    let ssf=self_subfields(& mut input);
    let rsf=rhs_subfields(& mut input);
    let impl0:syn::Expr=
        ssf.iter()
           .zip(rsf.iter())
           .map(|(l,r)|parse_quote!{#l.scalar_product(#r)})
           .reduce(|a,b|parse_quote!{#a + #b})
           .unwrap_or(parse_quote!{<ScProdT as num_traits::Zero>::zero()});
    let impl_try:syn::Expr=
        ssf.iter()
        .zip(rsf.iter())
        .map(|(l,r)|parse_quote!{#l.try_scalar_product(#r)?})
        .reduce(|a,b|parse_quote!{#a + #b})
        .unwrap_or(parse_quote!{<ScProdT as num_traits::Zero>::zero()});
    quote! {
        impl #generics #tr for #ty where ScProdT : num_traits::Zero, #(#types : #tr<ScProdT=ScProdT>,)* #wc {
            type ScProdT=ScProdT;
            fn scalar_product(self, rhs:Self) -> ScProdT {
                #impl0
            }
        }
        impl #generics #tr_try for #ty where ScProdT : num_traits::Zero, #(#types : #tr_try<TryScProdT=ScProdT>,)* #wc {
            type TryScProdT=ScProdT;
            fn try_scalar_product(self, rhs:Self) -> Option<ScProdT> {
                Some(#impl_try)
            }
        }
    }.into()
}


/// Implements [`algebra_traits::InnerProductSpace`]
/// 
/// inner product space
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::{InnerProductSpace};
/// 
/// #[derive(InnerProductSpace)]
/// struct MyWrapper<C>(C);
///
/// ```
#[proc_macro_derive(InnerProductSpace)]
pub fn inner_product_space_proc_macro(input: TokenStream) -> TokenStream {
    let fs=[
        is_a_zero_proc_macro,
        vectorspace_proc_macro,
        scalar_product_proc_macro,
        norm_proc_macro,
        norm_squared_proc_macro,
        distance_proc_macro];
    fs.iter()
      .map(|f|f(input.clone()))
      .collect()
}

/// Implements [`algebra_traits::ScalarDiv`] and Div<f64>
/// 
/// scalar division
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::ScalarDiv;
/// use algebra_traits::ScalarDiv;
///
/// #[derive(ScalarDiv, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper([0.0, 3.0, 6.0, 9.0]);
/// let wm=w.scalar_div(&3.0);
/// assert_eq!(wm, MyWrapper([0.0, 1.0, 2.0, 3.0]));
/// ```
#[proc_macro_derive(ScalarDiv)]
pub fn scalar_div_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::ScalarDiv<FScalarDiv>};
    let fn_name=parse_quote!{scalar_div};
    let (generics, wc, [(ty, types)],implementation)=
        DeriveHelper::new(& mut input, &tr,&fn_name)
            .add_gen_types(vec!["FScalarDiv"])
            .binary_const_rhs::<1>(false,&parse_quote!{f});
    quote! {
        impl #generics #tr for #ty where #(#types : #tr,)* #wc  {
            fn scalar_div(self, f:&FScalarDiv) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TryDiv`]
/// 
/// division with F
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::TryDiv;
/// use algebra_traits::TryDiv;
///
/// #[derive(TryDiv, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper([0.0, 3.0, 6.0, 9.0]);
/// let res=w.try_div(3_f64);
/// assert_eq!(res, Ok(MyWrapper([0.0, 1.0, 2.0, 3.0])));
/// ```
#[proc_macro_derive(TryDiv)]
pub fn try_div_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{algebra_traits::TryDiv};
    let struct_name=input.ident.clone();
    let (generics, wc, [(ty, mut wt),(ty1, mut wt1)])=
        preprocess_no_impl_add_gen_types(& mut input, vec!["FTryDiv"]);
    let wt=wt.remove(0);
    let wt1=wt1.remove(0);
    quote! {
        impl #generics #tr<FTryDiv> for #ty where #wt : #tr<FTryDiv,Output=#wt1>, #wc {
            type Output=#ty1;
            type Error=<#wt as #tr<FTryDiv>>::Error;
            fn is_divable_by(&self, f:&FTryDiv) -> Result<(),<#wt as #tr<FTryDiv>>::Error> {
                self.0
                    .is_divable_by(f)
            }

            fn try_div(self, f:FTryDiv) -> Result<#ty1,<#wt as #tr<FTryDiv>>::Error> {
                  self.0
                      .try_div(f)
                      .map(|d|#struct_name(d))
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TryDiv`]
/// 
/// division with F to self
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::TryDiv;
/// use algebra_traits::TryDiv;
///
/// #[derive(TryDiv, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper([0.0, 3.0, 6.0, 9.0]);
/// let res=w.try_div(3_f64);
/// assert_eq!(res, Ok(MyWrapper([0.0, 1.0, 2.0, 3.0])));
/// ```
#[proc_macro_derive(TryDivToSelf)]
pub fn try_div_to_self_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::TryDiv};
    let fn_name=parse_quote!{try_div};
    let (generics, wc, [(ty, types)],implementation)=
        DeriveHelper::new(& mut input, &tr,&fn_name)
            .add_gen_types(vec!["FTryDiv","ETryDiv"])
            .binary_const_rhs::<1>(true,&parse_quote!{f.clone()});
    let ssf=self_subfields(& mut input);
    quote! {
        impl #generics #tr<FTryDiv> for #ty where FTryDiv:Clone, #(#types : #tr<FTryDiv,Output=#types,Error=ETryDiv>,)* #wc {
            type Output=Self;
            type Error=ETryDiv;
            fn is_divable_by(&self, f:&FTryDiv) -> Result<(),ETryDiv> {
                #(#ssf.is_divable_by(f)?;)*
                Ok(())
            }

            fn try_div(self, f:FTryDiv) -> Result<Self,ETryDiv> {
                  #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TryDiv`] and [`algebra_traits::TryScalarDiv`]
/// 
/// safe scalar division with corresponding field,
/// requires T to implement [`algebra_traits::TryScalarDiv`]
/// returns None when trying to divide by zero
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::TryScalarDiv;
/// use algebra_traits::TryScalarDiv;
/// 
/// #[derive(TryScalarDiv, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let arr0=MyWrapper::<[f64;4]>([0.0, 4.0, 2.0,-4.0]);
/// let res=arr0.try_scalar_div(&2.0);
/// assert_eq!(res, Ok(MyWrapper([0.0, 2.0, 1.0,-2.0])));
///
/// let arr0=MyWrapper::<[f64;4]>([0.0, 4.0, 2.0,-4.0]);
/// let res=arr0.try_scalar_div(&0.0);
/// assert!(res.is_err());
/// ```
#[proc_macro_derive(TryScalarDiv)]
pub fn try_scalar_div_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::TryScalarDiv<Field>};
    let fn_name=parse_quote!{try_scalar_div};
    let (generics, wc, [(ty, wt)],implementation)=
        DeriveHelper::new(& mut input, &tr,&fn_name)
            .add_gen_types(vec!["Field","ETryScalarDiv"])
            .binary_const_rhs::<1>(true,&parse_quote!{f});
    quote! {
        impl #generics #tr for #ty where #(#wt : algebra_traits::TryScalarDiv<Field,Error=ETryScalarDiv>,)* #wc  {
            type Error=ETryScalarDiv;
            fn try_scalar_div(self, f:&Field) -> Result<Self,ETryScalarDiv> {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::AdditiveGroup`]
/// 
/// implements also [`num_traits::Zero`],
///                 [`std::ops::Neg`]
///                 [`std::ops::Add`]
///                 [`std::ops::Sub`]
///                 [`algebra_traits::TryAdd`]
///                 [`algebra_traits::TrySub`]
/// 
/// # Example
///
/// ```rust
/// use algebra_derive::AdditiveGroup;
///
/// #[derive(AdditiveGroup,Debug,PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let a=MyWrapper::<i32>(1);
/// let b=MyWrapper::<i32>(4);
/// let c=MyWrapper::<i32>(5);
/// 
/// assert_eq!(a+b,c);
/// 
/// ```
#[proc_macro_derive(AdditiveGroup)]
pub fn additive_group_proc_macro(input: TokenStream) -> TokenStream {
    let fs=
    [zero_proc_macro,
     closed_add_proc_macro,
     closed_sub_proc_macro,
     closed_neg_proc_macro,
     closed_try_add_proc_macro,
     closed_try_sub_proc_macro];
    fs.iter()
      .map(|f|f(input.clone()))
      .collect()
}

/// Implements [`algebra_traits::MultiplicativeGroup`]
/// 
/// implements also [`num_traits::One`],
///                 [`num_traits::Inv`]
///                 [`std::ops::Mul`]
///                 [`std::ops::Div`]
///  [`algebra_traits::TryMul`], [`algebra_traits::TryDiv`]
/// 
/// # Example
///
/// ```rust
/// use algebra_derive::MultiplicativeGroup;
///
/// #[derive(MultiplicativeGroup,Debug,PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let a=MyWrapper::<f64>(2.0);
/// let b=MyWrapper::<f64>(3.0);
/// let c=MyWrapper::<f64>(6.0);
/// 
/// assert_eq!(a*b,c);
/// 
/// ```
#[proc_macro_derive(MultiplicativeGroup)]
pub fn multiplicative_group_proc_macro(input: TokenStream) -> TokenStream {
    let fs=
    [one_proc_macro,
     closed_mul_proc_macro,
     closed_div_proc_macro,
     closed_inv_proc_macro];
    fs.iter()
      .map(|f|f(input.clone()))
      .collect()
}

/// Implements [`algebra_traits::Vectorspace`]
/// 
/// implements also [`algebra_traits::AdditiveGroup`],
///                 [`algebra_traits::ScalarMul`],
///                 [`algebra_traits::TryScalarDiv`]
/// # Example
///
/// ```rust
/// use algebra_derive::Vectorspace;
///
/// #[derive(Vectorspace)]
/// struct MyWrapper<T>(T);
/// 
/// ```
#[proc_macro_derive(Vectorspace)]
pub fn vectorspace_proc_macro(input: TokenStream) -> TokenStream {
    let fs=[
        additive_group_proc_macro,
        scalar_mul_proc_macro,
        try_scalar_div_proc_macro];
    fs.iter()
      .map(|f|f(input.clone()))
      .collect()
}

/// Implements [`algebra_traits::Basis`]
/// 
/// # Example
///
/// ```rust
/// use algebra_derive::Basis;
/// use algebra_traits::Basis;
/// 
/// #[derive(Basis)]
/// struct MyWrapper<C>(C);
/// 
/// let basis=<MyWrapper<[f64;3]> as Basis<f64>>::basis();
/// 
/// ```
#[proc_macro_derive(Basis)]
pub fn basis_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr:Path=parse_quote!{algebra_traits::Basis<Field>};
    let (generics, wc, [(ty, types)]) = preprocess_no_impl_add_gen_types(& mut input,vec!["Field"]);
    if types.len() != 1 {
        panic!{"derive macro Basis can only be used for structs with one field"};
    }
    let wt=&types[0];
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn basis() -> impl ExactSizeIterator<Item=Self> {
                <#wt as #tr>::basis()
                    .map(|b|Self(b))
            }
        }
    }.into()
}

/// Implements [`algebra_traits::FiniteDimensionalVectorspace`]
#[proc_macro_derive(FiniteDimensionalVectorspace)]
pub fn finite_dimensional_vectorspace_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{algebra_traits::FiniteDimensionalVectorspace<Field,FDIM>};
    let (generics, wc, [(ty, types)]) = preprocess_no_impl_add_gen_const_types(& mut input,vec!["Field"],vec!["FDIM"]);
    if types.len() != 1 {
        panic!{"derive macro FiniteDimensionalVectorspace can only be used for structs with one field"};
    }
    let wt=&types[0];
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {}
    }.into()
}

/// Implements [`algebra_traits::IsAZero`]
///
/// checks if its a zero
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::IsAZero;
/// use algebra_traits::IsAZero;
///
/// #[derive(IsAZero, Debug, PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let vec=MyWrapper(vec![0;4]);
/// assert_eq!(vec.is_a_zero(), true);
/// 
/// let vec=MyWrapper(vec![0, 1, 0, 0]);
/// assert_eq!(vec.is_a_zero(), false);
/// ```
#[proc_macro_derive(IsAZero)]
pub fn is_a_zero_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr:Path=parse_quote!{algebra_traits::IsAZero};
    let (generics, wc, [(ty, types)]) = preprocess_no_impl(& mut input);
    let self_subs=self_subfields(& mut input);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr,)* #wc  {
            fn is_a_zero(&self) -> bool {
                #(<#types as #tr>::is_a_zero(& #self_subs) && )* true
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TryAdd`]
///
/// # Example
/// 
/// ```rust
/// use algebra_traits::TryAdd;
/// use algebra_derive::TryAdd;
///
/// #[derive(TryAdd, Debug, PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// ```
#[proc_macro_derive(TryAdd)]
pub fn try_add_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::TryAdd};
    let fn_name=parse_quote!{try_add};
    let (generics, wc, [(ty, types),(ty1,types1), (ty2,types2)], implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["ETryAdd"]).for_try();
    let ssf=self_subfields(& mut input);
    let rsf=rhs_subfields(&mut input);
    quote! {
        impl #generics #tr<#ty1> for #ty where #(#types : #tr<#types1, Output=#types2, Error=ETryAdd>,)* #wc {
            type Output=#ty2;
            type Error=ETryAdd;
            fn is_addable_by(&self, rhs:&#ty1) -> Result<(),ETryAdd> {
                #(#ssf.is_addable_by(&#rsf)?;)*
                Ok(())
            }
            fn try_add(self, rhs : #ty1) -> Result<#ty2,ETryAdd> {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TryMul`]
///
/// # Example
/// 
/// ```rust
/// use algebra_traits::{TryMul,MulError,FloatOpError};
/// use algebra_derive::TryMul;
///
/// #[derive(TryMul, Debug, PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let a=MyWrapper(6);
/// let b=MyWrapper(7);
/// assert_eq!(a.try_mul(b),Ok(MyWrapper(42)));
/// 
/// let a=MyWrapper(99999999);
/// let b=MyWrapper(99999999);
/// assert_eq!(a.try_mul(b),Err(MulError::MathOp(FloatOpError::Overflow)));
/// ```
#[proc_macro_derive(TryMul)]
pub fn try_mul_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::TryMul};
    let fn_name=parse_quote!{try_mul};
    let (generics, wc, [(ty, types),(ty1,types1), (ty2,types2)], implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["ETryMul"]).for_try();
    let ssf=self_subfields(& mut input);
    let rsf=rhs_subfields(&mut input);
    quote! {
        impl #generics #tr<#ty1> for #ty where #(#types : #tr<#types1, Output=#types2, Error=ETryMul>,)* #wc {
            type Output=#ty2;
            type Error=ETryMul;
            fn is_mulable_by(&self, rhs:&#ty1) -> Result<(),ETryMul> {
                #(#ssf.is_mulable_by(#rsf)?;)*
                Ok(())
            }
            fn try_mul(self, rhs : #ty1) -> Result<#ty2, ETryMul> {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TryAdd`]  with Output=Self
/// 
/// try addition with itself
/// note : we can not simultaneously use derive_macro TryAdd and ClosedTryAdd,
/// TryAdd is a generalization of ClosedTryAdd
/// 
/// # Example
/// 
/// ```rust
/// use algebra_traits::TryAdd;
/// use algebra_derive::ClosedTryAdd;
///
/// #[derive(ClosedTryAdd, Debug, PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let vec0=MyWrapper(vec![4, 3, -4, 7]);
/// let vec1=MyWrapper(vec![1,-1,  4, 6]);
/// let res=vec0.try_add(vec1);
/// assert_eq!(res, Ok(MyWrapper(vec![5, 2, 0, 13])));
///
/// let vec0=MyWrapper(vec![4, 3, -4, 7]);
/// let vec1=MyWrapper(vec![1,-1,  4]);
/// let res=vec0.try_add(vec1);
/// assert!(res.is_err());
/// ```
#[proc_macro_derive(ClosedTryAdd)]
pub fn closed_try_add_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::TryAdd};
    let fn_name=parse_quote!{try_add};
    let (generics, wc, [(ty, types)], implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["ETryAdd"]).for_closed_try();
    let ssf=self_subfields(& mut input);
    let rsf=rhs_subfields(&mut input);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types, Error=ETryAdd> ,)* #wc {
            type Output=Self;
            type Error=ETryAdd;
            fn is_addable_by(&self, rhs:&Self) -> Result<(),ETryAdd> {
                #(<#types as #tr>::is_addable_by(&#ssf,&#rsf)?;)*
                Ok(())
            }
            fn try_add(self, rhs : Self) -> Result<Self,ETryAdd> {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TryMul`]  with Output=Self
/// 
/// try multiplication with itself
/// note : we can not simultaneously use derive_macro TryMul and ClosedTryMul,
/// TryMul is a generalization of ClosedTryMul
/// 
/// # Example
/// 
/// ```rust
/// use algebra_traits::TryMul;
/// use algebra_derive::ClosedTryMul;
///
/// #[derive(ClosedTryMul, Debug, PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let vec0=MyWrapper(6);
/// let vec1=MyWrapper(7);
/// let res=vec0.try_mul(vec1);
/// assert_eq!(res, Ok(MyWrapper(42)));
///
/// let vec0=MyWrapper(i32::MAX);
/// let vec1=MyWrapper(i32::MAX);
/// let res=vec0.try_mul(vec1);
/// assert!(res.is_err());
/// ```
#[proc_macro_derive(ClosedTryMul)]
pub fn closed_try_mul_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::TryMul};
    let fn_name=parse_quote!{try_mul};
    let (generics, wc, [(ty, types)], implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["ETryMul"]).for_closed_try();
    let ssf=self_subfields(& mut input);
    let rsf=rhs_subfields(&mut input);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types, Error=ETryMul>,)* #wc {
            type Output=Self;
            type Error=ETryMul;
            fn is_mulable_by(&self, rhs:&Self) -> Result<(),ETryMul> {
                #(<#types as #tr>::is_mulable_by(&#ssf,&#rsf)?;)*
                Ok(())
            }

            fn try_mul(self, rhs : Self) -> Result<Self,ETryMul> {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TryDiv`]  with Output=Self
/// 
/// try division with itself
/// 
/// # Example
/// 
/// ```rust
/// use algebra_traits::TryDiv;
/// use algebra_derive::ClosedTryDiv;
///
/// #[derive(ClosedTryDiv, Debug, PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let vec0=MyWrapper(42.0);
/// let vec1=MyWrapper(1.0);
/// let res=vec0.try_div(vec1);
/// assert_eq!(res, Ok(MyWrapper(42.0)));
///
/// let vec0=MyWrapper(42.0);
/// let vec1=MyWrapper(0.0);
/// let res=vec0.try_div(vec1);
/// assert!(res.is_err());
/// ```
#[proc_macro_derive(ClosedTryDiv)]
pub fn closed_try_div_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let struct_name=input.ident.clone();
    let (generics, wc, [(ty, wt)])=
        preprocess_no_impl_add_gen_types(& mut input,vec!["ETryDiv"]);
    let tr=quote!{algebra_traits::TryDiv };
    let ssf: Vec<syn::Expr>=self_subfields(& mut input);
    let rsf=rhs_subfields(&mut input);
    let exprs:Vec<syn::Expr>=
        wt.iter()
          .zip(ssf.iter())
          .zip(rsf.iter())
          .map(|((w,l),r)|parse_quote!{<#w as #tr>::try_div(#l,#r)?})
          .collect();
    let prep=Preprocessor::new(& mut input);
    let fields=prep.fields();
    let implementation=fields.struct_literal(&struct_name, exprs);
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr<Output=#wt, Error=ETryDiv>,)* #wc {
            type Output=Self;
            type Error=ETryDiv;
            fn is_divable_by(&self, rhs:&Self) -> Result<(),ETryDiv> {
                #(<#wt as #tr>::is_divable_by(&#ssf,&#rsf)?;)*
                Ok(())
            }
            fn try_div(self, rhs : Self) -> Result<Self,ETryDiv> {
                Ok(#implementation)
            }
        }
    }.into()
}



/// Implements [`algebra_traits::TrySub`]
///
/// # Example
/// 
/// ```rust
/// use algebra_traits::TrySub;
/// use algebra_derive::TrySub;
/// 
/// #[derive(PartialEq, Debug)]
/// struct Time(i32);
/// struct Duration(i32);
/// 
/// impl std::ops::Sub<Duration> for Time {
///     type Output=Time;
///     fn sub(self, rhs:Duration) -> Time {
///         Time(self.0-rhs.0)
///     }
/// }
/// algebra_traits::impl_anyop_from_op!(Sub<Duration>, sub, Time); 
///
/// #[derive(TrySub, Debug, PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let times:Vec<Time>=[4, 3, -4, 7].map(|i|Time(i)).into();
/// let times=MyWrapper(times);
/// 
/// let durs:Vec<Duration>=[1, -1,  4, 6].map(|i|Duration(i)).into();
/// let durs=MyWrapper(durs);
/// let res=times.try_sub(durs);
/// let exp:Vec<Time>=[3, 4,-8, 1].map(|i|Time(i)).into();
/// assert_eq!(res, Ok(MyWrapper(exp)));
///
/// let vec0=MyWrapper::<Vec<i32>>(vec![4, 3, -4, 7]);
/// let vec1=MyWrapper::<Vec<i32>>(vec![1,-1,  4]);
/// let res=vec0.try_sub(vec1);
/// assert!(res.is_err());
/// ```
#[proc_macro_derive(TrySub)]
pub fn try_sub_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::TrySub};
    let fn_name=parse_quote!{try_sub};
    let (generics, wc, [(ty, types),(ty1,types1), (ty2,types2)], implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["ETrySub"]).for_try();
    let ssf=self_subfields(& mut input);
    let rsf=rhs_subfields(&mut input);
    quote! {
        impl #generics #tr<#ty1> for #ty where #( #types : #tr<#types1, Output=#types2, Error=ETrySub>,)* #wc {
            type Output=#ty2;
            type Error=ETrySub;
            fn is_subable_by(&self, rhs:&#ty1) -> Result<(),ETrySub> {
                #(<#types as #tr<#types1>>::is_subable_by(&#ssf,&#rsf)?;)*
                Ok(())
            }
            fn try_sub(self, rhs:#ty1) -> Result<#ty2,ETrySub> {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TrySub`]  with Output=Self
/// 
/// try subtraction with itself
/// 
/// note : we can not simultaneously use derive_macro TrySub and ClosedTrySub,
/// TrySub is a generalization of ClosedTrySub
/// 
/// # Example
/// 
/// ```rust
/// use algebra_traits::TrySub;
/// use algebra_derive::ClosedTrySub;
///
/// #[derive(ClosedTrySub, Debug, PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let vec0:MyWrapper<Vec<i32>>=MyWrapper(vec![4, 3, -4, 7]);
/// let vec1:MyWrapper<Vec<i32>>=MyWrapper(vec![1,-1,  4, 6]);
/// let res=vec0.try_sub(vec1);
/// assert_eq!(res, Ok(MyWrapper(vec![3, 4, -8, 1])));
///
/// let vec0:MyWrapper<Vec<i32>>=MyWrapper(vec![4, 3, -4, 7]);
/// let vec1:MyWrapper<Vec<i32>>=MyWrapper(vec![1,-1,  4]);
/// let res=vec0.try_sub(vec1);
/// assert!(res.is_err());
/// ```
#[proc_macro_derive(ClosedTrySub)]
pub fn closed_try_sub_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::TrySub};
    let fn_name=parse_quote!{try_sub};
    let (generics, wc, [(ty, types)], implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).add_gen_types(vec!["ETrySub"]).for_closed_try();
    let ssf=self_subfields(& mut input);
    let rsf=rhs_subfields(&mut input);
    quote! {
        impl #generics #tr for #ty where #(#types : #tr<Output=#types, Error=ETrySub>,)* #wc {
            type Output=Self;
            type Error=ETrySub;
            fn is_subable_by(&self, rhs:&Self) -> Result<(),ETrySub> {
                #(<#types as #tr>::is_subable_by(&#ssf,&#rsf)?;)*
                Ok(())
            }
            fn try_sub(self, rhs : Self) -> Result<Self, ETrySub> {
                #implementation
            }
        }
    }.into()
}

/// Implements [`algebra:traits::Norm`]
///
/// norm of a vector
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::Norm;
/// use algebra_traits::Norm;
/// 
/// #[derive(Norm)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper<Vec<f64>>=MyWrapper(vec![3.0,4.0]);
/// assert_eq!(vec.norm(), 5.0);
/// ```
#[proc_macro_derive(Norm)]
pub fn norm_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, types)]) = preprocess_no_impl(& mut input);
    if types.len() !=1 {
        panic!("derive macro Norm only allowed for wrapper, i.e. struct with one field");
    }
    let wt=&types[0];
    let tr=quote!{algebra_traits::Norm};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            type NormT=<#wt as #tr>::NormT;
            fn norm(self) -> algebra_traits::Nonnegative<<#wt as #tr>::NormT> {
                <#wt as #tr>::norm(self.0)
            }
        }
    }.into()
}

/// Implements [`algebra_traits::NormSquared`]
///
/// squared norm of a vector
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::NormSquared;
/// use algebra_traits::NormSquared;
/// 
/// #[derive(NormSquared)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper<Vec<f64>>=MyWrapper(vec![3.0,4.0]);
/// assert_eq!(vec.norm_squared(),25.0);
/// ```
#[proc_macro_derive(NormSquared)]
pub fn norm_squared_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, types)]) = preprocess_no_impl(& mut input);
    if types.len() !=1 {
        panic!("derive macro NormSquared only allowed for wrapper, i.e. struct with one field");
    }
    let wt=&types[0];
    let tr=quote!{algebra_traits::NormSquared};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            type Norm2T=<#wt as #tr>::Norm2T;
            fn norm_squared(self) -> algebra_traits::Nonnegative<<#wt as #tr>::Norm2T> {
                <#wt as #tr>::norm_squared(self.0)
            }
        }
    }.into()
}

/// Implements [`algebra_traits::IsAZero`]
///            [`num_traits::Zero`]
///            [`std::ops::Add`]
///            [`std::ops::Sub`]
///            [`std::ops::Neg`]
///            [`algebra_traits::TryAdd`]
///            [`algebra_traits::TrySub`]
///            [`algebra_traits::ScalarMul`]
///            [`algebra_traits::TryScalarDiv`]
/// 
/// # Example
///
/// ```rust
/// use algebra_derive::Arithmetic;
///
/// #[derive(Arithmetic)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(Arithmetic)]
pub fn arithmetic_proc_macro(input: TokenStream) -> TokenStream {
    let fs=[
        is_a_zero_proc_macro,
        zero_proc_macro,
        neg_proc_macro,
        add_proc_macro,
        sub_proc_macro,
        try_add_proc_macro,
        try_sub_proc_macro,
        scalar_mul_proc_macro,
        try_scalar_div_proc_macro
    ];
    fs.iter()
      .map(|f|f(input.clone()))
      .collect()
}

/// Implements 
/// 
/// # Example
///
/// ```rust
/// use algebra_derive::MulVectorScalar;
///
/// #[derive(MulVectorScalar)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(MulVectorScalar)]
pub fn vector_mul_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name=input.ident.clone();
    let (generics, wc, [(ty, wt),(ty1,wt1)])=
        preprocess_no_impl_add_gen_types(& mut input,vec!["FRhs"]);
    let ssf=self_subfields(& mut input);
    let f=
        |s| if ssf.len() <= 1 { parse_quote!{#s * f} }  else  { parse_quote!{#s * f.clone() } };
    let cl=
    if ssf.len() <= 1 {
        quote!{}
    } else {
        quote!{ FRhs : Clone, }
    };
    let exprs=
        ssf.iter()
           .map(f)
           .collect();
    let prep=Preprocessor::new(& mut input);
    let fields=prep.fields();
    let implementation=fields.struct_literal(&struct_name, exprs);

    quote! {
        impl #generics std::ops::Mul<FRhs> for #ty where #(#wt : std::ops::Mul<FRhs,Output=#wt1>, )* #cl #wc {
            type Output=#ty1;
            fn mul(self, f:FRhs) -> #ty1 {
               #implementation
            }
        }
    }.into()
}

/// Implements 
/// 
/// # Example
///
/// ```rust
/// use algebra_derive::Vector;
///
/// #[derive(Vector)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(Vector)]
pub fn vector_proc_macro(input: TokenStream) -> TokenStream {
    let fs=[
        inner_product_space_proc_macro,
        conjugate_proc_macro,
        const_element_proc_macro,
        basis_proc_macro,
        vector_mul_proc_macro,
        finite_dimensional_vectorspace_proc_macro,
        try_div_proc_macro,
        try_normalize_proc_macro,
        cross_product_proc_macro,
    ];
    fs.iter()
      .map(|f|f(input.clone()))
      .collect()
}

/// Implements [`algebra_traits::Vectorspace`]
///            [`algebra_traits::Conjugate`]
/// 
/// # Example
///
/// ```rust
/// use algebra_derive::ScalarContainer;
///
/// #[derive(ScalarContainer)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(ScalarContainer)]
pub fn scalar_container_proc_macro(input: TokenStream) -> TokenStream {
    let fs=[
        vectorspace_proc_macro,
        conjugate_proc_macro
    ];
    fs.iter()
    .map(|f|f(input.clone()))
    .collect()
}


/// Implements [`algebra_traits::Crossproduct`]
/// 
/// cross product for 3d-vectors
/// # Example
/// 
/// ```rust
/// use algebra_derive::Crossproduct;
/// use algebra_traits::Crossproduct;
/// 
/// #[derive(Crossproduct, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let a:MyWrapper<[f64;3]>=MyWrapper([1.0,2.0,3.0]);
/// let b:MyWrapper<[f64;3]>=MyWrapper([4.0,5.0,6.0]);
/// assert_eq!(a.cross_product(b), MyWrapper([-3.0,6.0,-3.0]));
/// ```
#[proc_macro_derive(Crossproduct)]
pub fn cross_product_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{algebra_traits::Crossproduct};
    let fn_name=parse_quote!{cross_product};
    let (generics, wc, [(ty, types),(ty1,types1), (ty2,types2)],implementation)=
    DeriveHelper::new(& mut input,&tr,&fn_name).extended2();
    if types.len() !=1 {
        panic!("derive macro CrossProduct only allowed for wrapper, i.e. struct with one field");
    }
    quote! {
        impl #generics #tr<#ty1> for #ty where #(#types : #tr<#types1,Output=#types2>,)* #wc {
            type Output=#ty2;
            fn cross_product(self, rhs : #ty1) -> #ty2 {
                #implementation
            }
        }
    }.into()
}


/// implements ['algebra_traits::TryDiv<Rhs>`] with Rhs=<Self as [`algebra_traits::Norm`]>::NormT
///
/// # Example
/// 
#[proc_macro_derive(TryDivNorm)]
pub fn try_div_norm_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, types)])=preprocess_no_impl(& mut input);
    assert_eq!(types.len(),1);
    let wt=&types[0];
    let tr=quote!{ algebra_traits::TryDiv };
    let norm_t=quote!{ <#wt as algebra_traits::Norm>::NormT};
    quote! {
        impl #generics #tr<#norm_t> for #ty where #wt : algebra_traits::Norm+#tr<#norm_t,Output=#wt>, #wc {
            type Output=Self;
            type Error=<self as #tr<#norm_t>>::Error;
            fn try_div(self, rhs:#norm_t) -> Result<Self,<self as #tr<#norm_t>>::Error> {
                <#wt as #tr<#norm_t>>::try_div(self.0,rhs)
                    .map(|c|Self(c))
            }
        }
    }.into()
}

/// Implements [`algebra_traits::TryNormalize`]
///
/// # Example
/// 
/// ```rust
/// use algebra_derive::{Norm, TryDiv, TryNormalize};
/// use algebra_traits::{Norm, TryDiv, TryNormalize, DivError};
/// 
/// #[derive(Norm, TryDiv, TryNormalize, Clone, PartialEq, Debug)]
/// struct MyWrapper<T>(T);
/// 
/// let a:MyWrapper<[f64;2]>=MyWrapper([3.0,4.0]);
/// assert_eq!(a.try_divide_by_norm(), Ok((5.0,MyWrapper([0.6,0.8]))));
///
/// let a:MyWrapper<[f64;2]>=MyWrapper([0.0,0.0]);
/// assert_eq!(a.try_divide_by_norm(), Err(DivError::division_by_zero()));
/// ```
#[proc_macro_derive(TryNormalize)]
pub fn try_normalize_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{algebra_traits::TryNormalize};
    let (generics, wc, [(ty, wt)])=preprocess_no_impl(& mut input);
    assert_eq!(wt.len(),1);
    quote! {
        impl #generics #tr for #ty
            where Self : Clone
                        +algebra_traits::Norm
                        +algebra_traits::TryDiv<Self::NormT,Output=Self>, 
                  Self::NormT : Clone, #wc {}
    }.into()
}