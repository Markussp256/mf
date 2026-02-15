use proc_macro2::Span;
use syn::{parse_quote, DeriveInput, Generics, Ident};

use super::replace_type::ReplaceTypes;

// adds new params according to strings
pub fn generics_extended(derive_input:&DeriveInput, new_types:Vec<&str>) -> Generics {
    let mut generics=derive_input.generics.clone();
    for new_type in new_types {
        let new_type = Ident::new(new_type, Span::call_site());
        let new_type = parse_quote!{#new_type : 'static};
        generics.params.push(new_type);
    }
    generics
}


// for each pair of suffix and param adds a new param
pub fn generics_multiplied<'a>(derive_input:&'a mut DeriveInput, suffixes:Vec<&str>) -> (&'a mut Generics, Vec<ReplaceTypes>) {
    let mut generics=&mut derive_input.generics;
    let generics0=generics.clone();
    let rtys:Vec<ReplaceTypes>=
        suffixes.clone()
                .into_iter()
                .map(|suffix|ReplaceTypes::from_generics_and_suffix(&generics0,& mut generics, suffix))
                .collect();
    (generics, rtys)
}