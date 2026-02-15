use syn::{parse_quote, Expr, Ident, Index, Type};

pub trait Fields {
    fn len(&self) -> usize;
    fn subs(&self, expr:Expr) -> Vec<Expr>;
    fn types(&self) -> Vec<Type>;
    fn struct_literal(&self, struct_name:&Ident, exprs:Vec<Expr>) -> Expr;
}

trait Names {
    fn names(&self) -> impl Iterator<Item=Ident>;
}

impl Names for syn::FieldsNamed {
    fn names(&self) -> impl Iterator<Item=Ident> {
        self.named
            .iter()
            .map(|field|field.ident.clone().unwrap())
    }
}

impl Fields for syn::FieldsNamed {
    fn len(&self) -> usize {
        self.named
            .len()
    }

    fn types(&self) -> Vec<Type> {
        self.named
            .iter()
            .map(|name|name.ty.clone())
            .collect()
    }

    fn subs(&self, expr:Expr) -> Vec<Expr> {
        self.names()
            .map(move |name| parse_quote!{ #expr . #name })
            .collect()
    }

    fn struct_literal(&self, struct_name:&Ident, exprs:Vec<Expr>) -> Expr {
       let names=self.names();
       parse_quote!{ #struct_name { #( #names : #exprs ),* } }
    }
}

impl Fields for syn::FieldsUnnamed {

    fn len(&self) -> usize {
        self.unnamed
            .len()
    }

    fn types(&self) -> Vec<Type> {
        self.unnamed
            .iter()
            .map(|name|name.ty.clone())
            .collect()
    }

    fn subs(&self, expr:Expr) -> Vec<Expr> {
        (0..self.len())
            .into_iter()
            .map(Index::from)
            .map(move |i| parse_quote!{ #expr . #i })
            .collect()
    }


    fn struct_literal(&self, struct_name:&Ident, exprs:Vec<Expr>) -> Expr {
       parse_quote!{ #struct_name ( #( #exprs ),* ) }
    }
}


macro_rules! impl_fn {
    ($fn:ident (&self $(,$v_name:ident:$t:ty)*) -> $out:ty) => {
        fn $fn(&self $(,$v_name:$t)*) -> $out {
            match self {
                syn::Fields::Named(fields) => fields.$fn($($v_name),*),
                syn::Fields::Unnamed(fields) => fields.$fn($($v_name),*),
                _ => { panic!("Only named or unnamed struct allowed") }
            }
        }  
    }
}

impl Fields for syn::Fields {
    impl_fn!(len           (&self) -> usize);
    impl_fn!(types         (&self) -> Vec<Type>);
    impl_fn!(subs          (&self, expr:Expr) -> Vec<Expr>);
    impl_fn!(struct_literal(&self, struct_name:&Ident, exprs:Vec<Expr>) -> Expr);
}