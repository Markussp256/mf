use proc_macro2::Span;
use syn::{parse_quote, punctuated::Punctuated, token::Comma, Data, DeriveInput, Generics, Ident, Type, WherePredicate};
use quote::quote;

use super::{generics::generics_multiplied, fields_trait::Fields};

type OutNoImpl<const N:usize>=(proc_macro2::TokenStream, Punctuated<WherePredicate,Comma>, [(Type, Vec<Type>);N]);

pub fn preprocess_no_impl_add_gen_const_types<const N:usize>(
    input:& mut DeriveInput,
    add_gen_types:Vec<&'static str>,
    add_const_gen:Vec<&'static str>) -> OutNoImpl<N> {
    let mut prep=Preprocessor::new(input);
    let tys: [(Type, Vec<Type>); N]=prep.preprocess::<N>();
    prep.add_generic_types(add_gen_types);
    prep.add_const_generic(add_const_gen);
    let (igen, _, wc)=input.generics.split_for_impl();
    let igen=quote!{#igen};
    let wc=match wc {
        Some(wc) => wc.predicates.clone(),
        None => Punctuated::new()
    };
    (igen, wc, tys)
}

pub fn preprocess_no_impl_add_gen_types<const N:usize>(
    input:& mut DeriveInput,
    add_gen_types:Vec<&'static str>) -> OutNoImpl<N> {
    preprocess_no_impl_add_gen_const_types(input,add_gen_types,Vec::new())
}


pub fn preprocess_no_impl<const N:usize>(input:& mut DeriveInput) -> OutNoImpl<N> {
    preprocess_no_impl_add_gen_const_types(input,Vec::new(),Vec::new())
}


pub struct Preprocessor<'a> {
    input:& 'a mut DeriveInput
}

impl<'a> Preprocessor<'a> {
    pub fn new(input:&'a mut DeriveInput) -> Self {
        Self{input}
    }

    pub fn fields(&self) -> &syn::Fields {
        if let Data::Struct(data_struct)=&self.input.data {
            &data_struct.fields
        } else {
            panic!("Only struct allowed")
        }
    }

    fn get_types(&self) -> Vec<syn::Type> {
        self.fields()
            .types()
    }

    pub fn add_generic_types(& mut self, new_types:Vec<&str>) {
        for new_type in new_types {
            let new_type = Ident::new(new_type, Span::call_site());
            let new_type = parse_quote!{#new_type };
            self.input.generics.params.push(new_type);
        }
    }

    pub fn add_const_generic(& mut self, new_consts:Vec<&str>) {
        for new_const in new_consts {
            let const_ident = Ident::new(new_const, Span::call_site());
            let const_param = parse_quote!(const #const_ident: usize);
            self.input.generics.params.push(const_param);
        }
    }

    fn generics(&self) -> &Generics {
        &self.input
             .generics
    }

    fn get_type(&self) -> Type {
        let type_name=self.input.ident.clone();
        let gen=self.generics();
        let (_,t_gen,_)=gen.split_for_impl();
        if gen.params.is_empty() {
            parse_quote!{#type_name}
        } else {
            parse_quote!{#type_name :: #t_gen}
        }
    }


    pub fn preprocess<const N:usize>(& mut self) -> [(Type,Vec<Type>);N] {
        let ty=self.get_type();
        let types=self.get_types();
        let suffixes=match N {
            1 => Vec::new(),
            2 => vec!["Output"],
            3 => vec!["Rhs","Output"],
            _ => panic!("")
        };
        let (_, mut rty)=generics_multiplied(& mut self.input, suffixes);
        let tys:Vec<(Type,Vec<Type>)>=
            rty.iter_mut()
               .map(|rti|
                        (rti.gen_type(&ty),
                         types.iter()
                              .map(|t|rti.gen_type(t))
                              .collect()))
               .collect();
        utils::iter::next_chunk(& mut
            std::iter::once((ty,types))
                .chain(tys.into_iter())).ok().unwrap()
    }
}

