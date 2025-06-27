//! # container_derive
//!
//! This crate provides procedural macros to implement functions and/or traits for a wrapper.
//! 

use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, DeriveInput, Expr, Type, WherePredicate};
use quote::quote;

use derive_helper::{fields_trait::Fields, preprocessor::*, subfields, self_subfields, rhs_subfields, DeriveHelper};

fn preprocess(input:& mut DeriveInput) -> (proc_macro2::TokenStream, Punctuated<WherePredicate,Comma>, Type, Type) {
    let (gen_i, wc, [(ty, mut wts)])=preprocess_no_impl(input);
    let wt=wts.remove(0);
    (gen_i, wc, ty, wt)
}


/// Implements [`AsRef`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::AsRef;
/// 
/// #[derive(Debug, AsRef)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper(42);
/// assert_eq!(w.as_ref(), &42);
/// ```
#[proc_macro_derive(AsRef)]
pub fn as_ref_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, ty, wt) = preprocess(&mut input);
    quote! {
        impl #generics AsRef<#wt> for #ty where #wc {
            fn as_ref(&self) -> &#wt {
                &self.0
            }
        }
    }.into()
}

/// Implements [`container_traits::IntoInner`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::IntoInner;
/// use container_traits::IntoInner;
/// 
/// #[derive(IntoInner)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper(42);
/// assert_eq!(w.into_inner(), 42);
/// ```
#[proc_macro_derive(IntoInner)]
pub fn into_inner_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, ty, wt) = preprocess(& mut input);
    quote! {
        impl #generics container_traits::IntoInner for #ty where #wc {
            type InnerT=#wt;
            fn into_inner(self) -> #wt {
                self.0
            }
        }
    }.into()
}

/// Implements [`container_traits::Inner`]
/// 
/// Provides reference to inner type.
/// 
/// # Example
/// 
/// ```rust
/// use container_derive::Inner;
/// use container_traits::Inner;
/// 
/// #[derive(Inner)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper(42);
/// assert_eq!(w.inner(), &42);
/// ```
#[proc_macro_derive(Inner)]
pub fn inner_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, ty, wt) = preprocess(& mut input);
    quote! {
        impl #generics container_traits::Inner for #ty where #wc {
            type InnerT=#wt;
            fn inner(&self) -> &#wt {
                &self.0
            }
        }
    }.into()
}

/// Implements [`container_traits::FromInner`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::FromInner;
/// use container_traits::FromInner;
/// 
/// #[derive(FromInner, Debug, PartialEq)]
/// struct MyWrapper<T>(T);
/// 
/// let w=MyWrapper::<i32>::from_inner(42);
/// assert_eq!(w, MyWrapper(42));
/// ```
#[proc_macro_derive(FromInner)]
pub fn from_inner_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, ty, wt) = preprocess(& mut input);
    quote! {
        impl #generics container_traits::FromInner for #ty where #wc {
            type InnerT=#wt;
            fn from_inner(inner:#wt) -> Self {
                Self(inner)
            }
        }
    }.into()
}

/// Implements [`container_traits::TryAccept`]
/// 
/// # Example
///
/// ```rust
/// use container_derive::TryAccept;
/// use container_traits::{TryAccept, for_static};
///
/// #[derive(TryAccept)]
/// struct MyWrapper<C>(C);
/// 
/// let res:i32=7;
/// let f=|i:usize| &res;
/// 
/// assert!(MyWrapper::<Vec<i32>>::try_accept(3,&f).is_ok());
/// assert!(MyWrapper::<[i32;2] >::try_accept(2,&f).is_ok()); // not providing the right size would yield panic
/// assert!(<MyWrapper<[i32;3]>  as for_static  ::TryAccept<usize,i32>>::try_accept(  &f).is_ok());
/// 
/// ```
#[proc_macro_derive(TryAccept)]
pub fn try_accept_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, wt)]) = preprocess_no_impl_add_gen_types(& mut input,vec!["IndexTryAccept","TTryAccept","ErrorTryAccept"]);
    assert_eq!(wt.len(),1);
    let wt=&wt[0];
    let tr=    quote!{container_traits::             TryAccept<IndexTryAccept,TTryAccept,ErrorTryAccept>}; 
    let tr_arr=quote!{container_traits::for_static:: TryAccept<IndexTryAccept,TTryAccept,ErrorTryAccept>}; 
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn try_accept<'a>(size:IndexTryAccept,f:impl Fn(IndexTryAccept) -> &'a TTryAccept) -> Result<(),ErrorTryAccept> where TTryAccept : 'a {
                <#wt as #tr>::try_accept(size,f)
            }
        }

        impl #generics #tr_arr for #ty where #wt : #tr_arr, #wc {
            fn try_accept<'a>(f:impl Fn(IndexTryAccept) -> &'a TTryAccept) -> Result<(),ErrorTryAccept> where TTryAccept : 'a {
                <#wt as #tr_arr>::try_accept(f)
            }
        }

    }.into()
}

/// Implements [`container_traits::AsSlice`]
///
/// iterator for wrapper
///
/// # Example
/// 
/// ```rust
/// use container_derive::AsSlice;
/// use container_traits::AsSlice;
/// 
/// #[derive(AsSlice, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// let mut slice=vec.as_slice();
/// assert_eq!(slice,[0,1,2,3].as_slice());
/// ```
#[proc_macro_derive(AsSlice)]
pub fn as_slice_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);  
    let (generics, wc, ty, wt) = preprocess(& mut input);
    let tr=quote!{container_traits::AsSlice};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            type T=<#wt as #tr>::T;
            fn as_slice(&self) -> &[<#wt as #tr>::T] {
                <#wt as #tr>::as_slice(& self.0)
            }
        }
    }.into()
}

/// Implements [`container_traits::ItemT`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::ItemT;
/// use container_traits::ItemT;
/// 
/// #[derive(ItemT)]
/// struct MyWrapper<C>(C);
/// 
/// assert_eq!(std::any::TypeId::of::<<MyWrapper::<Vec<i32>> as ItemT>::T>(),
///            std::any::TypeId::of::<i32>());
/// ```
#[proc_macro_derive(ItemT)]
pub fn item_t_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{container_traits::ItemT};
    let (generics, wc, ty, wt) =preprocess(& mut input);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
           type T=<#wt as #tr>::T;
        }
    }.into()
}

/// Implements [`container_traits::IntoVec`]
///
///
/// # Example
/// 
/// ```rust
/// use container_derive::IntoVec;
/// use container_traits::IntoVec;
/// 
/// #[derive(IntoVec)]
/// struct MyWrapper<C>(C);
/// 
/// let v=MyWrapper(vec![1,2,3]);
/// assert_eq!(v.into_vec(),vec![1,2,3]);
/// 
/// let a=MyWrapper([1,2,3]);
/// assert_eq!(a.into_vec(),vec![1,2,3]);
/// ```
#[proc_macro_derive(IntoVec)]
pub fn into_vec_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{container_traits::IntoVec<TIntoVec> };
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TIntoVec"]);
    let implementation=if wt.len() == 0 {
        quote!{Vec::new()}
    } else {
        wt.iter()
          .zip(self_subfields(& mut input).iter())
          .map(|(w,ss)|quote!{<#w as #tr>::into_vec(#ss)})
          .reduce(|acc,new|quote!{container_traits::for_dynamic::Concat::concat(#acc,#new)})
          .unwrap()
    };
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr,)* #wc {
           fn into_vec(self) -> Vec<TIntoVec> {
                #implementation
           }
        }
    }.into()
}

/// Implements [`container_traits::Get`]
///
/// safe accessor for wrapper
///
/// # Example
/// 
/// ```rust
/// use container_derive::Get;
/// use container_traits::Get;
/// 
/// #[derive(Get, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// assert_eq!(vec.get(1), Some(&1_i32));
/// assert!(   vec.get(7).is_none());
/// ```
#[proc_macro_derive(Get)]
pub fn get_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{container_traits::Get<IndexGet,TGet>};
    let (generics, wc, [(ty, mut wt)]) =
    preprocess_no_impl_add_gen_types::<1>(& mut input, vec!["IndexGet","TGet"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn get(&self, index:IndexGet) -> Option<&TGet> {
                <#wt as #tr>::get(&self.0,index)
            }
        }
    }.into()
}

/// Implements [`container_traits::TryInsert`]
///
/// remove element
///
/// # Example
/// 
/// ```rust
/// use container_derive::TryInsert;
/// use container_traits::TryInsert;
/// 
/// #[derive(Get, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![7,4,2,3]);
/// assert!(vec.try_insert(1,42).is_ok())
/// assert_eq!(vec,MyWrapper(vec![7,42,4,2,3]));
/// ```
#[proc_macro_derive(TryInsert)]
pub fn try_insert_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{container_traits::TryInsert<TInsert>};
    let (generics, wc, [(ty, mut wt)]) =
    preprocess_no_impl_add_gen_types::<1>(& mut input, vec!["TInsert"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn try_insert(&mut self, index:usize, t:TInsert) -> Option<()> {
                <#wt as #tr>::try_insert(&mut self.0,index,t)
            }
        }
    }.into()
}

/// Implements [`container_traits::TryRemove`]
///
/// remove element
///
/// # Example
/// 
/// ```rust
/// use container_derive::TryRemove;
/// use container_traits::TryRemove;
/// 
/// #[derive(Get, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![7,4,2,3]);
/// assert_eq!(vec.try_remove(1), Some(4_i32));
/// assert_eq!(vec,MyWrapper(vec![7,2,3]));
/// ```
#[proc_macro_derive(TryRemove)]
pub fn try_remove_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{container_traits::TryRemove<TRemove>};
    let (generics, wc, [(ty, mut wt)]) =
    preprocess_no_impl_add_gen_types::<1>(& mut input, vec!["TRemove"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn try_remove(&mut self, index:usize) -> Option<TRemove> {
                <#wt as #tr>::try_remove(&mut self.0,index)
            }
        }
    }.into()
}

/// Implements [`container_traits::First`]
///
/// get first element
///
/// # Example
/// 
/// ```rust
/// use container_derive::First;
/// use container_traits::First;
/// 
/// #[derive(First, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// assert_eq!(vec.first(), Some(&0_i32));
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![]);
/// assert!(   vec.first().is_none());
/// ```
#[proc_macro_derive(First)]
pub fn first_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{container_traits::First<TFirst>};
    let (generics, wc, [(ty, mut wt)]) =
    preprocess_no_impl_add_gen_types::<1>(& mut input, vec!["TFirst"]);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn first(&self) -> Option<&TFirst> {
                <#wt as #tr>::first(&self.0)
            }
        }
    }.into()
}

/// Implements [`container_traits::Last`]
///
/// get last element
///
/// # Example
/// 
/// ```rust
/// use container_derive::Last;
/// use container_traits::Last;
/// 
/// #[derive(Last, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// assert_eq!(vec.last(), Some(&3_i32));
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![]);
/// assert!(   vec.last().is_none());
/// ```
#[proc_macro_derive(Last)]
pub fn last_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{container_traits::Last<TLast>};
    let (generics, wc, [(ty, mut wt)]) =
    preprocess_no_impl_add_gen_types::<1>(& mut input, vec!["TLast"]);
    let wt=wt.pop().unwrap();
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn last(&self) -> Option<&TLast> {
                <#wt as #tr>::last(&self.0)
            }
        }
    }.into()
}

/// Implements [`container_traits::GetMut`]
///
/// safe accessor for wrapper
///
/// # Example
/// 
/// ```rust
/// use container_derive::GetMut;
/// use container_traits::GetMut;
/// 
/// #[derive(GetMut, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// *vec.get_mut(1).unwrap() += 10;
/// assert_eq!(vec,MyWrapper(vec![0,11,2,3]));
/// ```
#[proc_macro_derive(GetMut)]
pub fn get_mut_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{container_traits::GetMut<IndexGetMut,TGetMut>};
    let (generics, wc, [(ty, mut wt)]) =
    preprocess_no_impl_add_gen_types::<1>(& mut input, vec!["IndexGetMut","TGetMut"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn get_mut(&mut self, index:IndexGetMut) -> Option<&mut TGetMut> {
                <#wt as #tr>::get_mut(&mut self.0, index)
            }
        }
    }.into()
}

/// Implements [`container_traits::Iter`]
///
/// iterator for wrapper
///
/// # Example
/// 
/// ```rust
/// use container_derive::Iter;
/// use container_traits::Iter;
/// 
/// #[derive(Iter, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// let mut iter=vec.iter();
/// assert_eq!(iter.next(), Some(&0_i32));
/// assert_eq!(iter.next(), Some(&1_i32));
/// assert_eq!(iter.next(), Some(&2_i32));
/// assert_eq!(iter.next(), Some(&3_i32));
/// assert!(iter.next().is_none());
/// ```
#[proc_macro_derive(Iter)]
pub fn iter_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{container_traits::Iter<TIter>};
    let (generics, wc, [(ty, wt)]) =
    preprocess_no_impl_add_gen_types::<1>(& mut input, vec!["TIter"]);
    let subfields=self_subfields(& mut input);
    let implementation=
        wt.iter()
          .zip(subfields)
          .map(|(wti,subfield)| quote!{<#wti as #tr>::iter(& #subfield )} )
          .reduce(|acc,new|quote!{<_ as utils::iter::ChainExactSize>::chain_exact_size(#acc,#new)})
          .unwrap_or(quote!{std::iter::empty()});
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr)*, #wc {
            fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a TIter> where TIter : 'a {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::iter::IndexedIter`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::IndexedIter;
/// use container_traits::IndexedIter;
/// 
/// #[derive(IndexedIter, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut iter=MyWrapper([7,2,5]).indexed_iter();
/// assert_eq!(iter.next(),Some((0,&7)));
/// assert_eq!(iter.next(),Some((1,&2)));
/// assert_eq!(iter.next(),Some((2,&5)));
/// assert_eq!(iter.next(),None);
/// 
/// ```
#[proc_macro_derive(IndexedIter)]
pub fn indexed_iter_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{ container_traits::IndexedIter<IndexIndexedIter,TIndexedIter> };
    let (generics, wc, [(ty, mut wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["IndexIndexedIter","TIndexedIter"]);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(IndexIndexedIter,&'a TIndexedIter)> where TIndexedIter : 'a {
                <#wt as #tr>::indexed_iter(&self.0)
            }
        }
    }.into()
}

/// Implements [`container_traits::AsMutSlice`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::AsMutSlice;
/// use container_traits::AsMutSlice;
/// 
/// #[derive(AsMutSlice, Debug, PartialEq)]
/// struct Wrapper<C>(C);
/// 
/// let mut vec:Wrapper::<Vec<i32>>=Wrapper(vec![0,1,2,3]);
/// let mut slice=vec.as_mut_slice();
/// slice[1]=42;
/// assert_eq!(vec,Wrapper(vec![0,42,2,3]));
/// ```
#[proc_macro_derive(AsMutSlice)]
pub fn as_mut_slice_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);  
    let (generics, wc, ty, wt) = preprocess(& mut input);
    let tr=quote!{container_traits::AsMutSlice};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            type T=<#wt as #tr>::T;
            fn as_mut_slice(&mut self) -> &mut [<#wt as #tr>::T] {
                <#wt as #tr>::as_mut_slice(& mut self.0)
            }
        }
    }.into()
}

/// Implements [`container_traits::IterMut`]
/// 
/// # Example
/// 
/// ```rust
/// use container_derive::IterMut;
/// use container_traits::IterMut;
/// 
/// #[derive(IterMut, PartialEq, Debug)]
/// struct MyWrapper<C>(C);
/// 
/// let mut vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
///
/// for vi in vec.iter_mut() {
///     *vi*=2;
/// }
///
/// assert_eq!(vec, MyWrapper(vec![0,2,4,6]));
/// ```
#[proc_macro_derive(IterMut)]
pub fn iter_mut_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);  
    let tr=quote!{container_traits::IterMut<TIterMut>};
    let (generics, wc, [(ty, wt)]) =
    preprocess_no_impl_add_gen_types::<1>(& mut input, vec!["TIterMut"]);
    let subfields=self_subfields(& mut input);
    let implementation=
        wt.iter()
          .zip(subfields)
          .map(|(wti,subfield)| quote!{<#wti as #tr>::iter_mut(& mut #subfield )} )
          .reduce(|acc,new|quote!{<_ as utils::iter::ChainExactSize>::chain_exact_size(#acc,#new)})
          .unwrap_or(quote!{std::iter::empty()});
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr)*, #wc {
            fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut TIterMut> where TIterMut : 'a {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::iter::IndexedIterMut`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::IndexedIterMut;
/// use container_traits::IndexedIterMut;
/// 
/// #[derive(IndexedIterMut, Debug, PartialEq)]
/// struct MyWrapper([i32;3]);
/// 
/// let mut a=MyWrapper([7,2,5]);
/// for (i, ai) in a.indexed_iter_mut() {
///   *ai += i as i32;
/// }
/// assert_eq!(a,MyWrapper([7,3,7]));
/// 
/// ```
#[proc_macro_derive(IndexedIterMut)]
pub fn indexed_iter_mut_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{ container_traits::IndexedIterMut<IndexIndexedIterMut,TIndexedIterMut> };
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["IndexIndexedIterMut", "TIndexedIterMut"]);
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr,)* #wc {
            fn indexed_iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(IndexIndexedIterMut,&'a mut TIndexedIterMut)> where TIndexedIterMut : 'a {
                self.0
                    .indexed_iter_mut()
            }
        }
    }.into()
}


/// Implements [`container_traits::for_dyn_and_stat::FromFn`]
///            [`container_traits::for_dyn_and_stat::TryFromFn`]
///            [`container_traits::for_static::FromFn`]
///            [`container_traits::for_static::TryFromFn`]
///
/// create from function
///
/// # Example
/// 
/// ```rust
/// use container_derive::FromFn;
/// use container_traits::{for_dyn_and_stat, for_static};
/// 
/// #[derive(Debug, PartialEq, FromFn)]
/// struct MyWrapper<C>(C);
/// 
/// let f=|i:usize| (i * i) as f64;
/// 
/// let vec=<MyWrapper<Vec<f64>> as for_dyn_and_stat::FromFn<usize,f64>>::from_fn(4,f);
/// assert_eq!(vec, MyWrapper(vec![0.0, 1.0, 4.0, 9.0]));
/// 
/// let arr=<MyWrapper<[f64;4]> as for_static::FromFn<usize,f64>>::from_fn(f);
/// assert_eq!(arr, MyWrapper([0.0, 1.0, 4.0, 9.0]));
/// ```
#[proc_macro_derive(FromFn)]
pub fn from_fn_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let mut input_any=input.clone();
    let (generics, wc, [(ty, mut wt)]) = preprocess_no_impl_add_gen_types(& mut input,vec!["IndexFromFn","TFromFn"]);
    let wt=wt.remove(0);
    let vec_tr=quote!{container_traits::FromFn<IndexFromFn,TFromFn>};
    let vec_try_tr=quote!{container_traits::TryFromFn<IndexFromFn,TFromFn,ErrorFromFn>};
    let arr_tr=quote!{container_traits::for_static::FromFn<IndexFromFn,TFromFn>};
    let arr_try_tr=quote!{container_traits::for_static::TryFromFn<IndexFromFn,TFromFn,ErrorFromFn>};
    let (generics_err, _, _) = preprocess_no_impl_add_gen_types::<1>(& mut input_any,vec!["IndexFromFn","TFromFn","ErrorFromFn"]);
    
    quote! {
        impl #generics #vec_tr for #ty where #wt : #vec_tr, #wc {
                fn from_fn(size:IndexFromFn, f: impl Fn(IndexFromFn) -> TFromFn) -> Self {
                    Self(<#wt as #vec_tr>::from_fn(size,f))
                }
        }

        impl #generics_err #vec_try_tr for #ty where #wt : #vec_try_tr, #wc {
            fn try_from_fn(size:IndexFromFn, f: impl Fn(IndexFromFn) -> TFromFn) -> Result<Self,ErrorFromFn> {
                <#wt as #vec_try_tr>::try_from_fn(size,f)
                     .map(|c|Self(c))
            }
        }

        impl #generics #arr_tr for #ty where #wt : #arr_tr, #wc {
                fn from_fn(f:impl Fn(IndexFromFn) -> TFromFn) -> Self {
                    Self(<#wt as #arr_tr>::from_fn(f))
                }
        }

        impl #generics_err #arr_try_tr for #ty where #wt : #arr_try_tr, #wc {
            fn try_from_fn(f:impl Fn(IndexFromFn) -> TFromFn) -> Result<Self,ErrorFromFn> {
                <#wt as #arr_try_tr>::try_from_fn(f)
                     .map(|c|Self(c))
            }
        }

    }.into()
}

/// Implements [`container_traits::for_dynamic::Empty`]
///
/// create empty Container
///
/// # Example
/// 
/// ```rust
/// use container_derive::Empty;
/// use container_traits::for_dynamic::Empty;
/// 
/// #[derive(Debug, PartialEq, Empty)]
/// struct MyWrapper<C>(C);
/// 
/// let vec=MyWrapper::<Vec<f64>>::empty();
/// assert_eq!(vec, MyWrapper(Vec::new()));
/// ```
#[proc_macro_derive(Empty)]
pub fn empty_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, ty, wt) = preprocess(& mut input);
    let tr=quote!{container_traits::for_dynamic::Empty};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
                fn empty() -> Self {
                    Self(<#wt as #tr>::empty())
                }

                fn is_empty(&self) -> bool {
                    self.0
                        .is_empty()
                }
        }
    }.into()
}

/// Implements [`container_traits::for_dynamic::OneElement`]
///
/// create empty Container
///
/// # Example
/// 
/// ```rust
/// use container_derive::OneElement;
/// use container_traits::for_dynamic::OneElement;
/// 
/// #[derive(Debug, PartialEq, OneElement)]
/// struct MyWrapper<C>(C);
/// 
/// let vec=MyWrapper::<Vec<i32>>::one_element(42);
/// assert_eq!(vec, MyWrapper(vec![42]));
/// ```
#[proc_macro_derive(OneElement)]
pub fn one_element_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, mut wt)]) = preprocess_no_impl_add_gen_types(& mut input,vec!["TOneElement"]);
    let wt=wt.remove(0);
    let tr=quote!{container_traits::for_dynamic::OneElement<TOneElement>};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn one_element(t:TOneElement) -> Self {
                Self(<#wt as #tr>::one_element(t))
            }
        }
    }.into()
}


/// Implements [`container_traits::for_dyn_and_stat::FromElement`]
///            [`container_traits::for_static::FromElement`]
///
/// create from element
///
/// # Example
/// 
/// ```rust
/// use container_derive::FromElement;
/// use container_traits::FromElement;
/// 
/// #[derive(FromElement, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper::from_element(4, 42);
/// assert_eq!(vec, MyWrapper(vec![42;4]));
/// ```
#[proc_macro_derive(FromElement)]
pub fn from_element_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let mut input_arr=input.clone();
    let (generics, wc, [(ty, mut wt)]) = preprocess_no_impl_add_gen_types(& mut input,vec!["IndexAnyFromElement","TFromElement"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    let vec_tr=quote!{container_traits::FromElement<IndexAnyFromElement,TFromElement> };
    let (generics_arr, _, _) = preprocess_no_impl_add_gen_types::<1>(& mut input_arr,vec!["TFromElement"]);
    let arr_tr=quote!{container_traits::for_static::FromElement<TFromElement> };

    let t_tb=quote!{ TFromElement : Clone };
    quote! {
        impl #generics #vec_tr for #ty where #wt : #vec_tr, #t_tb, #wc {
            fn from_element(size:IndexAnyFromElement, t:TFromElement) -> Self {
                Self(<#wt as #vec_tr>::from_element(size,t))
            }
        }

        impl #generics_arr #arr_tr for #ty where #wt : #arr_tr, #t_tb, #wc {
            fn from_element(t:TFromElement) -> Self {
                Self(<#wt as #arr_tr>::from_element(t))
            }
        }
    }.into()
}

/// Implements [`container_traits::TryIntoElement`]
///
/// move into element
///
/// # Example
/// 
/// ```rust
/// use container_derive::TryIntoElement;
/// use container_traits::TryIntoElement;
/// 
/// #[derive(TryIntoElement, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,42,3]);
/// assert_eq!(vec.try_into_element(2), Some(42));
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,42,3]);
/// assert_eq!(vec.try_into_element(5), None);
/// ```
#[proc_macro_derive(TryIntoElement)]
pub fn try_into_element_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, mut wt)]) = preprocess_no_impl_add_gen_types(& mut input,vec!["IndexTryIntoElement","TTryIntoElement"]);
    let wt=wt.remove(0);
    let tr=quote!{container_traits::TryIntoElement<IndexTryIntoElement,TTryIntoElement>};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn try_into_element(self, index:IndexTryIntoElement) -> Option<TTryIntoElement> {
                <#wt as #tr>::try_into_element(self.0, index)
            }
        }
    }.into()
}

/// Implements [`container_traits::Enumerate`]
///
/// numerate elements
///
/// # Example
/// 
/// ```rust
/// use container_derive::Enumerate;
/// use container_traits::Enumerate;
/// 
/// #[derive(Enumerate, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,42,3]);
/// assert_eq!(vec.enumerate(), MyWrapper(vec![(0,0),(1,1),(2,42),(3,3)]));
/// ```
#[proc_macro_derive(Enumerate)]
pub fn enumerate_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{container_traits::Enumerate};
    let fn_name=parse_quote!{enumerate};
    let (generics, wc, [(ty,mut wts),(ty1,mut wts1)],implementation)=
       DeriveHelper::new(& mut input, &tr, &fn_name).extended1();
    assert_eq!(wts.len(),1);
    let wt=wts.remove(0);
    let wt1=wts1.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr<Output=#wt1>, #wc {
            type Output=#ty1;
            fn enumerate(self) -> #ty1 {
                #implementation
            }
        }
    }.into()
}

/// Implements [`container_traits::Zeros`]
///
/// create container with all elements equal to zero
///
/// # Example
/// 
/// ```rust
/// use container_derive::Zeros;
/// use container_traits::Zeros;
/// 
/// #[derive(Zeros, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper::zeros(4);
/// assert_eq!(vec, MyWrapper(vec![0;4]));
/// ```
#[proc_macro_derive(Zeros)]
pub fn zeros_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, mut wt)]) = preprocess_no_impl_add_gen_types(& mut input,vec!["IndexZeros","TZeros"]);
    let wt=wt.remove(0);
    let tr=quote!{container_traits::Zeros<IndexZeros,TZeros>};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn zeros(size:IndexZeros) -> Self where TZeros : num_traits::Zero {
                Self(<#wt as #tr>::zeros(size))
            }
        }
    }.into()
}


/// Implements [`container_traits::Size`]
///            [`container_traits::OCTSize`]
/// 
/// returns size of container
///
/// # Example
/// 
/// ```rust
/// use container_derive::Size;
/// use container_traits::{Size,for_static};
/// 
/// #[derive(Size, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper<Vec<i32>>=MyWrapper(vec![0;4]);
/// assert_eq!(vec.size(),     4);
/// 
/// let arr:MyWrapper<[i32;3]>=MyWrapper([0,1,2]);
/// assert_eq!(<MyWrapper::<[i32;3]> as for_static::Size<usize>>::SIZE, 3);
/// ```
#[proc_macro_derive(Size)]
pub fn size_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, mut wt)]) = preprocess_no_impl_add_gen_types(& mut input,vec!["IndexSize"]);
    let wt=wt.remove(0);
    let vec_tr=quote!{container_traits::Size<IndexSize>};
    let arr_tr=quote!{container_traits::for_static::Size<IndexSize>};
    let oct_tr=quote!{container_traits::OCTSize<IndexSize>};

    quote! {
        impl #generics #vec_tr for #ty where #wt : #vec_tr, #wc {
            fn size(&self) -> IndexSize {
                <#wt as #vec_tr>::size(&self.0)
            }
        }

        impl #generics #arr_tr for #ty where #wt : #arr_tr, #wc {
            const SIZE:IndexSize=<#wt as #arr_tr>::SIZE;
        }

        impl #generics #oct_tr for #ty where #wt : #oct_tr, #wc {
            const OCTSIZE:Option<IndexSize>=<#wt as #oct_tr>::OCTSIZE;
        }

    }.into()
}

/// Implements [`container_traits::NumberOfDegreesOfFreedom`]
///            [`container_traits::for_static::NumberOfDegreesOfFreedom`]
///
/// returns number of degrees of freedom of class instance
///
/// # Example
/// 
/// ```rust
/// use container_derive::NumberOfDegreesOfFreedom;
/// use container_traits::NumberOfDegreesOfFreedom;
/// 
/// #[derive(NumberOfDegreesOfFreedom, Debug, PartialEq)]
/// struct MyWrapper<T>(Vec<T>,Vec<T>);
/// 
/// let vec=MyWrapper(vec![1,2],vec![1,2,3]);
/// assert_eq!(vec.ndofs(), 5);
/// ```
#[proc_macro_derive(NumberOfDegreesOfFreedom)]
pub fn ndofs_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, wt)]) = preprocess_no_impl_add_gen_types(& mut input,vec!["TNDOFS"]);
    let vec_tr=quote!{container_traits::NumberOfDegreesOfFreedom<TNDOFS> };
    let arr_tr=quote!{container_traits::for_static::NumberOfDegreesOfFreedom<TNDOFS> };
    let ssf=self_subfields(& mut input);
    quote! {
        impl #generics #vec_tr for #ty where #(#wt : #vec_tr,)* #wc {
            fn ndofs(&self) -> usize {
                0 #(+ <#wt as #vec_tr>::ndofs(& #ssf))*
            }
        }

        impl #generics #arr_tr for #ty where #(#wt : #arr_tr,)* #wc {
           const NDOFS:usize=0 #(+ <#wt as #arr_tr>::NDOFS)*;
        }
    }.into()
}

/// Implements [`container_traits::Map`]
///
/// maps container to container with possibly different associated type.
///
/// # Example
/// 
/// ```rust
/// use container_derive::Map;
/// use container_traits::Map;
/// 
/// #[derive(Map, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// let mapped_vec=vec.map(|v|2*v);
/// assert_eq!(mapped_vec, MyWrapper(vec![0,2,4,6]));
/// ```
#[proc_macro_derive(Map)]
pub fn map_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{container_traits::Map<TIn,TOut> };
    let fn_name=parse_quote!{map};
    let (generics,wc,[(ty, wt),(ty1,wt1)],implementation)=
        DeriveHelper::new(& mut input,&tr,&fn_name)
                .add_gen_types(vec!["TIn","TOut"])
                .binary_const_rhs::<2>(false, &parse_quote!{&f});
    quote! {
        impl #generics #tr for #ty where #(#wt : container_traits::Map<TIn,TOut, Output=#wt1>)*, #wc {
                type Output=#ty1;
                fn map(self, f: impl Fn(TIn) -> TOut) -> #ty1 {
                    #implementation
                }
        }
    }.into()
}

/// Implements [`container_traits::TryMap`]
///
/// maps container to container with possibly different associated type.
///
#[proc_macro_derive(TryMap)]
pub fn try_map_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{container_traits::TryMap<TIn,TOut,ETryMap> };
    let fn_name=parse_quote!{try_map};
    let (generics,wc,[(ty, wt),(ty1,wt1)],implementation)=
        DeriveHelper::new(& mut input,&tr,&fn_name)
                .add_gen_types(vec!["TIn","TOut","ETryMap"])
                .binary_const_rhs::<2>(true, &parse_quote!{&f});
    quote! {
        impl #generics #tr for #ty where #(#wt : container_traits::TryMap<TIn,TOut, ETryMap, Output=#wt1>)*, #wc {
                type Output=#ty1;
                fn try_map(self, f: impl Fn(TIn) -> TOut) -> Result<#ty1,ETryMap> {
                    #implementation
                }
        }
    }.into()
}

/// Implements [`container_traits::ClosedMap`]
///
/// maps container to self by applying function to each element.
///
/// # Example
/// 
/// ```rust
/// use container_derive::ClosedMap;
/// use container_traits::Map;
/// 
/// #[derive(ClosedMap, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// let mapped_vec=vec.map(|v|2*v);
/// assert_eq!(mapped_vec, MyWrapper(vec![0,2,4,6]));
/// ```
#[proc_macro_derive(ClosedMap)]
pub fn closed_map_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{container_traits::Map<TClosedMap,TClosedMap> };
    let fn_name=parse_quote!{map};
    let (generics,wc,[(ty, wt)],implementation)=
        DeriveHelper::new(& mut input,&tr,&fn_name)
                .add_gen_types(vec!["TClosedMap"])
                .binary_const_rhs::<1>(false, &parse_quote!{&f});
    quote! {
        impl #generics #tr for #ty where #(#wt : container_traits::ClosedMap<TClosedMap,Output=#wt>)*, #wc {
            type Output=Self;
            fn map(self, f: impl Fn(TClosedMap) -> TClosedMap) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`container_traits::TryClosedMap`]
///
/// maps container to container with possibly different associated type.
///
#[proc_macro_derive(TryClosedMap)]
pub fn try_closed_map_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=parse_quote!{container_traits::TryMap<TTryClosedMap,TTryClosedMap,ErrorTryClosedMap> };
    let fn_name=parse_quote!{try_map};
    let (generics,wc,[(ty, wt)],implementation)=
        DeriveHelper::new(& mut input,&tr,&fn_name)
                .add_gen_types(vec!["TTryClosedMap","ErrorTryClosedMap"])
                .binary_const_rhs::<1>(true, &parse_quote!{&f});
    quote! {
        impl #generics #tr for #ty where #(#wt : container_traits::TryClosedMap<TTryClosedMap,ErrorTryClosedMap>)*, #wc {
            type Output=Self;
            fn try_map(self, f: impl Fn(TTryClosedMap) -> TTryClosedMap) -> Result<Self,ErrorTryClosedMap> {
                #implementation
            }
        }
    }.into()
}


/// Implements [`std::iter::Extend`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::Extend;
///
/// #[derive(Extend, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut vec:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// vec.extend([4,5,6]);
/// assert_eq!(vec, MyWrapper(vec![0,1,2,3,4,5,6]));
/// ```
#[proc_macro_derive(Extend)]
pub fn extend_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty,mut wt)])=preprocess_no_impl_add_gen_types(& mut input,vec!["TExtend"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    quote! {
        impl #generics Extend<TExtend> for #ty where #wt : Extend<TExtend>, #wc {
                fn extend<I:IntoIterator<Item=TExtend>>(& mut self, iter: I) {
                    self.0
                        .extend(iter)
            }
        }
    }.into()
}

/// Implements [`container_traits::Concat`]
///
/// concatenates two vec wrappers
///
/// # Example
/// 
/// ```rust
/// use container_derive::Concat;
/// use container_traits::for_dynamic::Concat;
/// 
/// #[derive(Concat, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec0:MyWrapper::<Vec<i32>>=MyWrapper(vec![0,1,2,3]);
/// let vec1:MyWrapper::<Vec<i32>>=MyWrapper(vec![4,5,6]);
/// let concat_vec=vec0.concat(vec1);
/// assert_eq!(concat_vec, MyWrapper(vec![0,1,2,3,4,5,6]));
/// ```
#[proc_macro_derive(Concat)]
pub fn concat_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, ty, wt) = preprocess(& mut input);
    let tr=quote!{container_traits::for_dynamic::Concat};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn concat(self, rhs: Self) -> Self {
                Self(<#wt as #tr>::concat(self.0,rhs.0))
            }
        }
    }.into()
}

/// Implements [`container_traits::TryPutAt`]
///            [`container_traits::for_static::TryPutAt`]
///
/// create instance where i-th element is t and the rest is 0
///
/// # Example
/// 
/// ```rust
/// use container_derive::TryPutAt;
/// use container_traits::TryPutAt;
/// 
/// #[derive(TryPutAt, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let v=MyWrapper::<Vec<f64>>::try_put_at(4,2,42.0);
/// assert_eq!(v, Ok(MyWrapper(vec![0.0,0.0,42.0,0.0])));
/// ```
#[proc_macro_derive(TryPutAt)]
pub fn try_put_at_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, mut wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["IndexTryPutAT","TTryPutAt"]);
    let wt=wt.remove(0);
    let vec_tr=quote!(container_traits::TryPutAt<IndexTryPutAT,TTryPutAt>);
    let arr_tr=quote!(container_traits::for_static::TryPutAt<IndexTryPutAT,TTryPutAt>);
    let out=quote!{Result<Self,container_traits::IndexOutOfBoundsError<IndexTryPutAT>>};
    quote! {
        impl #generics #vec_tr for #ty where #wt : #vec_tr, TTryPutAt : num_traits::Zero, #wc {
            fn try_put_at(size:IndexTryPutAT, index:IndexTryPutAT, t:TTryPutAt) -> #out {
                <#wt as #vec_tr>::try_put_at(size,index,t)
                    .map(|s|Self(s))
            }
        }

        impl #generics #arr_tr for #ty where #wt : #arr_tr, TTryPutAt : num_traits::Zero, #wc {
            fn try_put_at(index:IndexTryPutAT, t:TTryPutAt) -> #out {
                <#wt as #arr_tr>::try_put_at(index,t)
                    .map(|s|Self(s))
            }
        }


    }.into()
}


/// Implements [`container_traits::StandardBasis`]
///            [`container_traits::for_static::StandardBasis`]
///
/// create basis for wrapper where i-th element is 1 and the rest is 0
///
/// # Example
/// 
/// ```rust
/// use container_derive::StandardBasis;
/// use container_traits::StandardBasis;
/// 
/// #[derive(StandardBasis, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut basis=MyWrapper::<Vec<f64>>::standard_basis(2);
/// assert_eq!(basis, vec![MyWrapper(vec![1.0,0.0]), MyWrapper(vec![0.0,1.0])]);
/// ```
#[proc_macro_derive(StandardBasis)]
pub fn standard_basis_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, ty, wt)=preprocess(& mut input);
    let vec_tr=quote!(container_traits::StandardBasis);
    let arr_tr=quote!(container_traits::for_static::StandardBasis);
    quote! {
        impl #generics #vec_tr for #ty where #wt : #vec_tr, #wc {
            fn try_standard_basis_element(len:usize, index:usize) -> Result<Self,container_traits::IndexOutOfBoundsError<usize>> {
                <#wt as #vec_tr>::try_standard_basis_element(len,index)
                    .map(|s|Self(s))
            }
        }

        impl #generics #arr_tr for #ty where #wt : #arr_tr, #wc {
            fn try_standard_basis_element(index:usize) -> Result<Self,container_traits::IndexOutOfBoundsError<usize>> {
                <#wt as #arr_tr>::try_standard_basis_element(index)
                    .map(|s|Self(s))
            }

            fn standard_basis() -> impl ExactSizeIterator<Item=Self> {
                <#wt as #arr_tr>::standard_basis()
                    .map(|s|Self(s))
            }
        }

    }.into()
}

/// Implements [`container_traits::for_dynamic::FromVec`]
///            [`container_traits::AnyFromVec`]
///            [`container_traits::for_static::TryFromVec`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::FromVec;
/// use container_traits::FromVec;
/// 
/// #[derive(FromVec, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut v=MyWrapper::<Vec<i32>>::from_vec(vec![1,2]);
/// assert_eq!(v, MyWrapper(vec![1,2]));
/// ```
#[proc_macro_derive(FromVec)]
pub fn from_vec_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let mut input_any=input.clone();
    let (generics, wc, [(ty, wt)])=preprocess_no_impl_add_gen_types(& mut input,vec!["TFromVec"]);
    assert_eq!(wt.len(),1);
    let wt=&wt[0];
    let vec_tr=quote!(container_traits::for_dynamic::FromVec<TFromVec>);

    let (generics_any,_, _)=preprocess_no_impl_add_gen_types::<1>(& mut input_any,vec!["TFromVec","ErrorFromVec"]);
    let any_tr=quote!(container_traits::AnyFromVec<TFromVec,ErrorFromVec>);
    let try_tr=quote!{container_traits::TryFromVec<TFromVec,ErrorFromVec>};
    quote! {
        impl #generics_any #any_tr for #ty where #wt : #any_tr, #wc {
            fn any_from_vec(v:Vec<TFromVec>) -> Result<Self,ErrorFromVec> {
                <#wt as #any_tr>::any_from_vec(v)
                    .map(|s|Self(s))
            }
        }

        impl #generics_any #try_tr for #ty where #wt : #try_tr, #wc {
            fn try_from_vec(v:Vec<TFromVec>) -> Result<Self,ErrorFromVec> {
                <#wt as #try_tr>::try_from_vec(v)
                    .map(|s|Self(s))
            }
        }

        impl #generics #vec_tr for #ty where #wt : #vec_tr, #wc {
            fn from_vec(v:Vec<TFromVec>) -> Self {
                Self(<#wt as #vec_tr>::from_vec(v))
            }
        }
    }.into()
}


/// Implements [`container_traits::Reverse`]
///
/// reverses the order of the elements
///
/// # Example
/// 
/// ```rust
/// use container_derive::Reverse;
/// use container_traits::Reverse;
/// 
/// #[derive(Reverse, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let vec:MyWrapper<Vec<i32>>=MyWrapper(vec![0, 1, 2, 3]);
/// assert_eq!(vec.reverse(), MyWrapper(vec![3, 2, 1, 0]));
/// ```
#[proc_macro_derive(Reverse)]
pub fn reverse_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, ty, wt) = preprocess(& mut input);
    let tr=quote!{container_traits::Reverse};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
                fn reverse(self) -> Self {
                    Self(<#wt as #tr>::reverse(self.0))
                }
        }
    }.into()
}

/// Implements [`container_traits::for_dynamic::Pop`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::Pop;
/// use container_traits::for_dynamic::Pop;
/// 
/// #[derive(Pop, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut vec:MyWrapper<Vec<i32>>=MyWrapper(vec![0, 1, 2, 3]);
/// assert_eq!(vec.pop(), Some(3));
/// assert_eq!(vec, MyWrapper(vec![0, 1, 2]));
/// ```
#[proc_macro_derive(Pop)]
pub fn pop_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TPop"]);
    assert_eq!(wt.len(),1);
    let wt=&wt[0];
    let tr=quote!{container_traits::for_dynamic::Pop<TPop>};
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn pop(& mut self) -> Option<TPop> {
                <#wt as #tr>::pop(&mut self.0)
            }
        }
    }.into()
}

/// Implements [`container_traits::for_dynamic::Push`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::Push;
/// use container_traits::for_dynamic::Push;
/// 
/// #[derive(Push, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut vec:MyWrapper<Vec<i32>>=MyWrapper(vec![0, 1, 2, 3]);
/// vec.push(4);
/// assert_eq!(vec, MyWrapper(vec![0, 1, 2, 3, 4]));
/// ```
#[proc_macro_derive(Push)]
pub fn push_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TPush"]);
    let tr=quote!{container_traits::for_dynamic::Push<TPush>};
    assert_eq!(wt.len(),1);
    let wt=&wt[0];
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn push(& mut self, t:TPush) {
                <#wt as #tr>::push(&mut self.0, t)
            }
        }
    }.into()
}

/// Implements  [`container_traits::AnyFromIterator`]
///             [`container_traits::for_static::TryFromIterator`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::{IntoIterator,TryFromIterator};
/// use container_traits::for_static::TryFromIterator;
///
/// #[derive(IntoIterator, TryFromIterator, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let res=<MyWrapper::<[i32;3]> as TryFromIterator<i32,_>>::try_from_iter(vec![0, 1, 2]);
/// assert_eq!(res, Ok(MyWrapper([0, 1, 2])));
/// 
/// let res=MyWrapper::<[i32;3]>::try_from_iter(vec![0, 1, 2, 3]);
/// assert!(res.is_err());
/// ```
#[proc_macro_derive(TryFromIterator)]
pub fn try_from_iter_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name=input.ident.clone();
    let tr=quote!{ container_traits::for_static::TryFromIterator<TTryFromIter,ErrorTryFromIter> };
    let tr_any=quote!{ container_traits::AnyFromIterator<TTryFromIter, ErrorTryFromIter> };
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TTryFromIter","ErrorTryFromIter"]);
    let mut input_c=input.clone();
    let prep=Preprocessor::new(& mut input);
    let fields=prep.fields();
    let exprs:Vec<Expr>=
        wt.iter()
          .map(|wti|parse_quote!{ <#wti as #tr>::try_take_away(iter)? })
          .collect();
    let implementation=fields.struct_literal(&struct_name,exprs);
    let exprs_r=subfields(& mut input_c, parse_quote!{ r });
    let exprs_any:Vec<Expr>=
        wt.iter()
          .zip(exprs_r.iter())
          .map(|(wti,ri)|parse_quote!{ <#wti as #tr_any>::any_take_away(oref.map(|r|&#ri),iter)? })
          .collect();
    let implementation_any=fields.struct_literal(&struct_name,exprs_any);
    quote! {
        impl #generics #tr_any for #ty where #(#wt : #tr_any,)* Self : container_traits::IntoIter<TTryFromIter>, container_traits::LenNotEqualToRequiredLenError : Into<ErrorTryFromIter>, #wc {
            fn any_take_away<I:Iterator<Item=TTryFromIter>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,ErrorTryFromIter> {
                Ok(#implementation_any)
            }

            container_traits::any_from_iter_impl!(TTryFromIter, ErrorTryFromIter);
        }

        impl #generics #tr for #ty where #(#wt : #tr,)* Self : container_traits::IntoIter<TTryFromIter>, container_traits::LenNotEqualToRequiredLenError : Into<ErrorTryFromIter>, #wc {
            fn try_take_away<I:Iterator<Item=TTryFromIter>>(iter:& mut I) -> Result<Self,ErrorTryFromIter> {
                Ok(#implementation)
            }

            container_traits::try_from_iter_impl!(TTryFromIter, ErrorTryFromIter);
        }
    }.into()
}

/// Implements  [`container_traits::TryFromLocalParameters`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::TryFromLocalParameters;
/// use container_traits::TryFromLocalParameters;
///
/// #[derive(TryFromLocalParameters, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let a=MyWrapper::<f64>(1.0);
/// let res=a.try_from_iter(vec![0.2]);
/// assert_eq!(res, Ok(MyWrapper(1.2)));
/// ```
#[proc_macro_derive(TryFromLocalParameters)]
pub fn try_from_local_parameters_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name=input.ident.clone();
    let tr=quote!{ container_traits::TryFromLocalParameters<TTryFromIter,ErrorTryFromIter> };
    let ssf=self_subfields(& mut input);
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TTryFromIter","ErrorTryFromIter"]);
    let prep=Preprocessor::new(& mut input);
    let fields=prep.fields();
    let exprs:Vec<Expr>=
        wt.iter()
          .zip(ssf.iter())
          .map(|(wti,ssfi)|parse_quote!{ <#wti as #tr>::try_take_away(#ssfi, iter)? })
          .collect();
    let implementation=fields.struct_literal(&struct_name,exprs);
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr,)* container_traits::LenNotEqualToRequiredLenError : Into<ErrorTryFromIter>, #wc {
            fn try_take_away<I:Iterator<Item=TTryFromIter>>(self,iter:& mut I) -> Result<Self,ErrorTryFromIter> {
                Ok(#implementation)
            }

            container_traits::try_from_local_parameters_impl!(TTryFromIter, ErrorTryFromIter);
        }
    }.into()
}

/// Implements  [`container_traits::AnyFromParameters`]
///             [`container_traits::for_static::TryFromParameters`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::{IntoParameters,TryFromParameters};
/// use container_traits::for_static::TryFromParameters;
///
/// #[derive(IntoParameters, TryFromParameters, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let res=<MyWrapper::<[i32;3]> as TryFromParameters<i32,_>>::try_from_iter(vec![0, 1, 2]);
/// assert_eq!(res, Ok(MyWrapper([0, 1, 2])));
/// 
/// let res=MyWrapper::<[i32;3]>::try_from_iter(vec![0, 1, 2, 3]);
/// assert!(res.is_err());
/// ```
#[proc_macro_derive(TryFromParameters)]
pub fn try_from_parameters_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name=input.ident.clone();
    let tr=quote!{ container_traits::for_static::TryFromParameters<TTryFromIter,ErrorTryFromIter> };
    let tr_any=quote!{ container_traits::AnyFromParameters<TTryFromIter, ErrorTryFromIter> };
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TTryFromIter","ErrorTryFromIter"]);
    let mut input_c=input.clone();
    let prep=Preprocessor::new(& mut input);
    let fields=prep.fields();
    let exprs:Vec<Expr>=
        wt.iter()
          .map(|wti|parse_quote!{ <#wti as #tr>::try_take_away(iter)? })
          .collect();
    let implementation=fields.struct_literal(&struct_name,exprs);
    let exprs_r=subfields(& mut input_c, parse_quote!{ r });
    let exprs_any:Vec<Expr>=
        wt.iter()
          .zip(exprs_r.iter())
          .map(|(wti,ri)|parse_quote!{ <#wti as #tr_any>::any_take_away(oref.map(|r|&#ri),iter)? })
          .collect();
    let implementation_any=fields.struct_literal(&struct_name,exprs_any);
    quote! {
        impl #generics #tr_any for #ty where #(#wt : #tr_any,)* Self : container_traits::IntoParameters<TTryFromIter>, container_traits::LenNotEqualToRequiredLenError : Into<ErrorTryFromIter>, #wc {
            fn any_take_away<I:Iterator<Item=TTryFromIter>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,ErrorTryFromIter> {
                Ok(#implementation_any)
            }

            container_traits::any_from_parameters_impl!(TTryFromIter, ErrorTryFromIter);
        }

        impl #generics #tr for #ty where #(#wt : #tr,)* Self : container_traits::IntoParameters<TTryFromIter>, container_traits::LenNotEqualToRequiredLenError : Into<ErrorTryFromIter>, #wc {
            fn try_take_away<I:Iterator<Item=TTryFromIter>>(iter:& mut I) -> Result<Self,ErrorTryFromIter> {
                Ok(#implementation)
            }

            container_traits::try_from_parameters_impl!(TTryFromIter, ErrorTryFromIter);
        }
    }.into()
}

/// Implements [`std::iter::FromIterator`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::FromIterator;
///
/// #[derive(FromIterator, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let res=MyWrapper::<Vec<i32>>::from_iter(vec![0, 1, 2]);
/// assert_eq!(res, MyWrapper(vec![0, 1, 2]));
/// ```
#[proc_macro_derive(FromIterator)]
pub fn from_iter_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name=input.ident.clone();
    let tr=quote!{ FromIterator<TFromIter> };
    let (generics, wc, [(ty, mut wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TFromIter"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    let exprs=vec![parse_quote!{<#wt as #tr>::from_iter(iter)}];
    let prep=Preprocessor::new(& mut input);
    let fields=prep.fields();
    let implementation=fields.struct_literal(&struct_name,exprs);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn from_iter<I:IntoIterator<Item=TFromIter>>(iter:I) -> Self {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::iter::IntoIterator`]
///       [`container_traits::IntoIter`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::IntoIterator;
///
/// #[derive(IntoIterator, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut iter=MyWrapper([1,2,3]).into_iter();
/// assert_eq!(iter.next(),Some(1));
/// assert_eq!(iter.next(),Some(2));
/// assert_eq!(iter.next(),Some(3));
/// assert_eq!(iter.next(),None);
/// 
/// ```
#[proc_macro_derive(IntoIterator)]
pub fn into_iter_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{ container_traits::IntoIter<TIntoIter> };
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TIntoIter"]);
    let ssf=self_subfields(& mut input);
    let implementation:Expr=
        ssf.iter()
           .map(|s|parse_quote!{<_ as #tr>::into_iterator(#s)})
           .reduce(|a,b|parse_quote!{ utils::iter::ChainExactSize::chain_exact_size(#a,#b) })
           .unwrap_or(parse_quote!{std::iter::empty()} );
    let impl2:Expr=
    ssf.iter()
       .map(|s|parse_quote!{<_ as IntoIterator>::into_iter(#s)})
       .reduce(|a,b|parse_quote!{ #a.chain(#b) })
       .unwrap_or(parse_quote!{std::iter::empty()} );
    // let empty:Type=parse_quote!{std::iter::Empty<TIntoIter> };
    let into_iter_t=
        wt.iter()
          .map(|t|parse_quote!{<#t as IntoIterator>::IntoIter})
          .reduce(|a:Type,b:Type|parse_quote!{ std::iter::Chain<#a ,#b> })
          .unwrap_or(parse_quote!{ std::iter::Empty<TIntoIter> });
    
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr,)* #wc {
            fn into_iterator(self) -> impl ExactSizeIterator<Item=TIntoIter> {
                #implementation
            }
        }

        impl #generics IntoIterator for #ty where #(#wt : IntoIterator<Item=TIntoIter>,)* #wc {
            type Item=TIntoIter;
            type IntoIter=#into_iter_t;
            fn into_iter(self) -> Self::IntoIter {
                #impl2
            }
        }

    }.into()
}

/// Implements [`container_traits::IntoParameters`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::IntoParameters;
/// use container_traits::IntoParameters;
/// 
/// #[derive(IntoParameters, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut iter=MyWrapper([1,2,3]).into_parameters();
/// assert_eq!(iter.next(),Some(1));
/// assert_eq!(iter.next(),Some(2));
/// assert_eq!(iter.next(),Some(3));
/// assert_eq!(iter.next(),None);
/// 
/// ```
#[proc_macro_derive(IntoParameters)]
pub fn into_parameters_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{ container_traits::IntoParameters<TIntoParameters> };
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TIntoParameters"]);
    let ssf=self_subfields(& mut input);
    let implementation=
        ssf.iter()
           .map(|s| parse_quote!{ #s.into_parameters() })
           .reduce(|a:Expr,b:Expr|parse_quote!{ utils::iter::ChainExactSize::chain_exact_size(#a,#b) })
           .unwrap_or(parse_quote!{ std::iter::empty() } );
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr,)* #wc {
            fn into_parameters(self) -> impl ExactSizeIterator<Item=TIntoParameters> {
                #implementation
            }
        }
    }.into()
}

/// Implements [`container_traits::IntoLocalParameters`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::IntoLocalParameters;
/// use container_traits::IntoLocalParameters;
///
/// #[derive(IntoLocalParameters, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut iter=MyWrapper(1.0).into_local_parameters(MyWrapper(1.2));
/// assert_eq!(iter.next(),Some(1.2-1.0));
/// assert_eq!(iter.next(),None);
/// 
/// ```
#[proc_macro_derive(IntoLocalParameters)]
pub fn into_local_parameters_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{ container_traits::IntoLocalParameters<TIntoLocalParameters> };
    let (generics, wc, [(ty, wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TIntoLocalParameters"]);
    let ssf=self_subfields(& mut input);
    let rsf=rhs_subfields(& mut input);
    let implementation=
        ssf.iter()
           .zip(rsf.iter())
           .map(|(s,r)|parse_quote!{#s.into_local_parameters(#r)})
           .reduce(|a:Expr,b:Expr|parse_quote!{ utils::iter::ChainExactSize>::chain_exact_size(#a, #b) })
           .unwrap_or(parse_quote!{std::iter::empty()});
    quote! {
        impl #generics #tr for #ty where #(#wt : #tr,)* #wc {
            fn into_local_parameters(self,rhs:Self) -> impl ExactSizeIterator<Item=TIntoLocalParameters> {
                #implementation
            }
        }
    }.into()
}

/// Implements [`std::iter::IntoIndexedIter`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::IntoIndexedIter;
/// use container_traits::IntoIndexedIter;
/// 
/// #[derive(IntoIndexedIter, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let mut iter=MyWrapper([7,2,5]).into_indexed_iter();
/// assert_eq!(iter.next(),Some((0,7)));
/// assert_eq!(iter.next(),Some((1,2)));
/// assert_eq!(iter.next(),Some((2,5)));
/// assert_eq!(iter.next(),None);
/// 
/// ```
#[proc_macro_derive(IntoIndexedIter)]
pub fn into_indexed_iter_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr=quote!{ container_traits::IntoIndexedIter<IndexIntoIndexedIter,TIntoIndexedIter> };
    let (generics, wc, [(ty, mut wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["IndexIntoIndexedIter","TIntoIndexedIter"]);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr for #ty where #wt : #tr, #wc {
            fn into_indexed_iter(self) -> impl ExactSizeIterator<Item=(IndexIntoIndexedIter,TIntoIndexedIter)> {
                <#wt as #tr>::into_indexed_iter(self.0)
            }
        }
    }.into()
}

/// Implements [`container_traits::XYZ`]
///
/// # Example
/// 
/// ```rust
/// use container_derive::XYZ;
/// use container_traits::for_static::{X,Y,Z};
///
/// #[derive(XYZ, Debug, PartialEq)]
/// struct MyWrapper<C>(C);
/// 
/// let v=MyWrapper([1.0;2.0]);
/// assert_eq!(v.x(),&1.0);
/// assert_eq!(v.y(),&2.0);
/// assert_eq!(MyWrapper::ex(),MyWrapper([1.0;0.0]));
/// assert_eq!(MyWrapper::ey(),MyWrapper([0.0,1.0]));
/// 
/// ```
#[proc_macro_derive(XYZ)]
pub fn xyz_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let tr_x=quote!{ container_traits::for_static::X<TXYZ> };
    let tr_y=quote!{ container_traits::for_static::Y<TXYZ> };
    let tr_z=quote!{ container_traits::for_static::Z<TXYZ> };
    let (generics, wc, [(ty, mut wt)])=
    preprocess_no_impl_add_gen_types(& mut input,vec!["TXYZ"]);
    let wt=wt.remove(0);
    quote! {
        impl #generics #tr_x for #ty where #wt : #tr_x, #wc {
            fn x(&self) -> &TXYZ {
                <#wt as #tr_x>::x(&self.0)
            }

            fn ex() -> Self where TXYZ : num_traits::Zero + num_traits::One {
                Self(<#wt as #tr_x>::ex())
            }
        }
        impl #generics #tr_y for #ty where #wt : #tr_y, #wc {
            fn y(&self) -> &TXYZ {
                <#wt as #tr_y>::y(&self.0)
            }

            fn ey() -> Self where TXYZ : num_traits::Zero + num_traits::One {
                Self(<#wt as #tr_y>::ey())
            }
        }
        impl #generics #tr_z for #ty where #wt : #tr_z, #wc {
            fn z(&self) -> &TXYZ {
                <#wt as #tr_z>::z(&self.0)
            }

            fn ez() -> Self where TXYZ : num_traits::Zero + num_traits::One {
                Self(<#wt as #tr_z>::ez())
            }
        }
    }.into()
}

/// Implements traits that are implemented by vec but not by array type
/// 
/// Implements [`container_traits::for_dynamic::Concat`]
///            [`container_traits::for_dynamic::Empty`]
///            [`std::iter::Extend`]
///            [`container_traits::for_dynamic::IntoSub`]
///            [`container_traits::for_dynamic::PadZeros`]
///            [`container_traits::for_dynamic::Pop`]
///            [`container_traits::for_dynamic::Push`]
///            [`container_traits::for_dynamic::Zeros`]
/// 
/// # Example
///
/// ```rust
/// use container_derive::ForDyn;
///
/// #[derive(ForDyn)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(ForDyn)]
pub fn for_dyn_proc_macro(input: TokenStream) -> TokenStream {
    let fs=[
        concat_proc_macro,
        empty_proc_macro,
        one_element_proc_macro,
        extend_proc_macro,
        pop_proc_macro,
        push_proc_macro,
        try_insert_proc_macro,
        try_remove_proc_macro,
        zeros_proc_macro
    ];
    fs.iter()
      .map(|f|f(input.clone()))
      .collect()
}

/// Implements [`container_traits::Container`]
/// 
/// # Example
///
/// ```rust
/// use container_derive::JustContainer;
///
/// #[derive(JustContainer)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(JustContainer)]
pub fn just_container_proc_macro(input: TokenStream) -> TokenStream {
    [  
        get_proc_macro,
        ndofs_proc_macro,
        iter_proc_macro,
        into_iter_proc_macro,
        indexed_iter_proc_macro,
        into_indexed_iter_proc_macro,
        into_vec_proc_macro,
        size_proc_macro,
        first_proc_macro,
        last_proc_macro,
        item_t_proc_macro,
        try_into_element_proc_macro
    ].iter()
     .map(|f|f(input.clone()))
     .collect()
}

/// Implements [`container_traits::ContainerTryConstruct`]
/// 
/// implements also ContainerTryConstruct if the wrapped type implements the necessary traits
/// # Example
///
/// ```rust
/// use container_derive::ContainerTryConstruct;
///
/// #[derive(ContainerTryConstruct)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(ContainerTryConstruct)]
pub fn container_any_construct_proc_macro(input: TokenStream) -> TokenStream {
    [  
        just_container_proc_macro,
        try_accept_proc_macro,
        from_vec_proc_macro,
        from_fn_proc_macro,
        closed_map_proc_macro,
        try_closed_map_proc_macro,
    ].iter()
     .map(|f|f(input.clone()))
     .collect()
}

/// Implements [`container_traits::ContainerMut`]
/// 
/// # Example
///
/// ```rust
/// use container_derive::ContainerMut;
///
/// #[derive(ContainerMut)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(ContainerMut)]
pub fn container_mut_proc_macro(input: TokenStream) -> TokenStream {
    [  
        just_container_proc_macro,
        iter_mut_proc_macro,
        get_mut_proc_macro
    ].iter()
     .map(|f|f(input.clone()))
     .collect()
}

/// Implements [`container_traits::ContainerDynamic`]
/// 
/// # Example
///
/// ```rust
/// use container_derive::ContainerDynamic;
///
/// #[derive(ContainerDynamic)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(ContainerDynamic)]
pub fn container_dynamic_proc_macro(input: TokenStream) -> TokenStream {
    [  
        container_any_construct_proc_macro,
        empty_proc_macro,
        one_element_proc_macro,
        pop_proc_macro,
        push_proc_macro,
        try_insert_proc_macro,
        try_remove_proc_macro,
        extend_proc_macro
    ].iter()
     .map(|f|f(input.clone()))
     .collect()
}

/// Implements traits (or pairs of traits) that are implemented for dynamic and static types 
/// 
/// Implements [`container_traits::for_static::TryFromIterator`]
///            [`std::iter::FromIterator`]
///            [`container_traits::AnyFromIterator`]
///            [`container_traits::AsMutSlice`]
///            [`container_traits::AsSlice`]
///            [`container_traits::ChangeT`]
///            [`container_traits::Enumerate`]
///            [`container_traits::for_static::FromElement`]
///            [`container_traits::FromElement`]
///            [`container_traits::for_static::FromFn`]
///            [`container_traits::for_static::TryFromFn`]
///            [`container_traits::FromFn`]
///            [`container_traits::TryFromFn`]
///            [`container_traits::for_static::TryFromVec`]
///            [`container_traits::for_dynamic::FromVec`]
///            [`container_traits::AnyFromVec`]
///            [`container_traits::FromInner`]
///            [`container_traits::IntoInner`]
///            [`container_traits::Get`]
///            [`container_traits::GetMut`]
///            [`std::iter::IntoIterator`]
///            [`container_traits::IterMut`]
///            [`container_traits::Iter`]
///            [`container_traits::Len`]
///            [`container_traits::IsLenPossible`]
///            [`container_traits::for_static::NumberOfDegreesOfFreedom`]
///            [`container_traits::NumberOfDegreesOfFreedom`]
///            [`container_traits::IsNDoFPossible`]
///            [`container_traits::Map`]
///            [`container_traits::Reverse`]
///            [`container_traits::TryIntoElement`]
///            [`container_traits::for_static::StandardBasis`]
///            [`container_traits::StandardBasis`]
///            [`container_traits::Container`]
///            [`container_traits::ContainerTryConstruct`]
///            [`container_traits::ContainerMut`]
///            [`container_traits::ContainerDynamic`]
/// 
/// # Example
///
/// ```rust
/// use container_derive::Container;
///
/// #[derive(Container)]
/// struct MyWrapper<C>(C);
/// 
/// ```
#[proc_macro_derive(Container)]
pub fn container_proc_macro(input: TokenStream) -> TokenStream {
    let fs=[
        try_from_iter_proc_macro,
        try_from_parameters_proc_macro,
        from_iter_proc_macro,
        as_mut_slice_proc_macro,
        as_slice_proc_macro,
        change_t_proc_macro,
        enumerate_proc_macro,
        for_dyn_proc_macro,
        from_element_proc_macro,
        from_fn_proc_macro,
        from_vec_proc_macro,
        from_inner_proc_macro,
        inner_proc_macro,
        into_inner_proc_macro,
        get_proc_macro,
        first_proc_macro,
        last_proc_macro,
        get_mut_proc_macro,
        into_iter_proc_macro,
        into_parameters_proc_macro,
        into_indexed_iter_proc_macro,
        into_vec_proc_macro,
        item_t_proc_macro,
        iter_mut_proc_macro,
        indexed_iter_mut_proc_macro,
        iter_proc_macro,
        indexed_iter_proc_macro,
        try_accept_proc_macro,
        size_proc_macro,
        ndofs_proc_macro,
        map_proc_macro,
        try_map_proc_macro,
        reverse_proc_macro,
        try_into_element_proc_macro,
        try_put_at_proc_macro,
        standard_basis_proc_macro,
        xyz_proc_macro,
    ];
    fs.iter()
      .map(|f|f(input.clone()))
      .collect()
}

/// Implements [`container_traits::ChangeT`]
///
/// changes base type of container
///
/// # Example
/// 
/// ```rust
/// use container_derive::ChangeT;
/// use container_traits::ChangeT;
/// 
/// #[derive(ChangeT)]
/// struct MyWrapper<C>(C);
/// 
/// assert_eq!(std::any::TypeId::of::<<MyWrapper<Vec<f64>> as ChangeT<i32>>::Output>(),
///            std::any::TypeId::of::<MyWrapper<Vec<i32>>>());
/// ```
#[proc_macro_derive(ChangeT)]
pub fn change_t_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (generics, wc, [(ty,mut wt),(ty1,mut wt1)])=
    preprocess_no_impl_add_gen_types(& mut input, vec!["T2ChangeT"]);
    assert_eq!(wt.len(),1);
    let wt=wt.remove(0);
    let wt1=wt1.remove(0);
    quote! {
        impl #generics container_traits::ChangeT<T2ChangeT> for #ty
        where #wt : container_traits::ChangeT<T2ChangeT,Output=#wt1>, #wc {
            type Output=#ty1;
        }
    }.into()
}

/// Implements      [`algebra_traits::Parameter<F>`]
/// 
#[proc_macro_derive(Parameter)]
pub fn parameters1_proc_macro(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let tr=quote!{algebra_traits::Parameter<FParameter>};
    let (generics, wc, [(ty, types)])=preprocess_no_impl_add_gen_types(& mut input, vec!["FParameter"]);
    assert_eq!(types.len(),1);
    let wt=&types[0];
    quote! {
        impl #generics algebra_traits::Parameter<FParameter> for #ty where Self : Clone, #wt : #tr, #wc {
            fn parameters(&self) -> FParameter {
                <#wt as #tr>::parameter(&self.0)
            }

            fn from_parameter(f:FParameter) -> Self {
                Self(<#wt as #tr>::from_parameter(f))
            }
        }
    }.into()
}