
use syn::{parse_quote, Data, DeriveInput, Expr, Ident, Path};
use super::fields_trait::Fields;

use super::derive_helper::{Arity, FnInfo};

fn append_try(expr: Expr) -> Expr {
    // Create an ExprTry by wrapping the original expression with a ?
    Expr::Try(syn::ExprTry {
        attrs: Vec::new(),
        expr: Box::new(expr),
        question_token: syn::token::Question {
            spans: [proc_macro2::Span::call_site()],
        },
    })
}

trait MaybeAddQM {
    fn maybe_add_qm(self, add_qm:bool) -> Vec<Expr>;
}

impl<I:Iterator<Item=Expr>> MaybeAddQM for I {
    fn maybe_add_qm(self, add_qm:bool) -> Vec<Expr> {
        if add_qm {
            self.map(append_try)
                .collect()
        } else {
            self.collect()
        }
    }
}

pub struct Implementor<'a> {
    input:&'a DeriveInput,
    tr:&'a Path,
    fn_name:&'a Ident,
    is_try:bool
}

impl<'a> Implementor<'a> {
    pub fn new(input:&'a DeriveInput,tr:&'a Path,fn_name:&'a Ident,is_try:bool) -> Self {
        Self{input,tr,fn_name,is_try}
    }

    fn fields(&self) -> &syn::Fields {
        if let Data::Struct(data_struct)=&self.input.data {
            &data_struct.fields
        } else {
            panic!("Only struct allowed")
        }
    }

    fn types(&self) -> Vec<syn::Type> {
        self.fields()
            .types()
    }

    fn struct_name(&self) -> &Ident {
        &self.input
             .ident
    }

    fn tr_info(&self) -> (&Path, &Ident) {
       (&self.tr, &self.fn_name)
    }


    fn nullary_const(&self) -> Vec<Expr> {
        let (tr,fn_name)=self.tr_info();
        self.fields()
            .types()
            .into_iter()
            .map(|t|parse_quote!{<#t as #tr>:: #fn_name })
            .collect()
    }

    fn nullary_exprs(&self) -> Vec<Expr> {
        let (tr,fn_name)=self.tr_info();
        self.fields()
            .types()
            .into_iter()
            .map(|t|parse_quote!{<#t as #tr>:: #fn_name () })
            .collect()
    }

    fn unary_exprs(&self) -> Vec<Expr> {
        let (tr,fn_name)=self.tr_info();
        self.fields()
            .subs(parse_quote!(self))
            .into_iter()
            .zip(self.types())
            .map(|(s,t)|parse_quote!{ <#t as #tr>::#fn_name(#s)})
            .collect()
    }

    fn unary_const_arg(&self, arg:&Expr) -> Vec<Expr> {
        let (tr,fn_name)=self.tr_info();
        self.types()
            .into_iter()
            .map(|t|parse_quote!{ <#t as #tr>::#fn_name(#arg)})
            .maybe_add_qm(self.is_try)
    }

    fn binary_const_rhs(&self, rhs:&Expr) -> Vec<Expr> {
        let (tr,fn_name)=self.tr_info();
        self.fields()
            .subs(parse_quote!(self))
            .into_iter()
            .zip(self.types())
            .map(|(s,t)|parse_quote!{ <#t as #tr>::#fn_name(#s, #rhs) })
            .maybe_add_qm(self.is_try)
    }

    fn binary_exprs(&self) -> Vec<Expr> {
        let fields=self.fields();
        let (tr,fn_name)=self.tr_info();
        let s=fields.subs(parse_quote!(self));
        let rhs=fields.subs(parse_quote!{rhs});
            s.into_iter()
             .zip(rhs)
             .map(|(s,rhs)|parse_quote!{ <_ as #tr<_>>::#fn_name(#s,#rhs) })
             .maybe_add_qm(self.is_try)
    }

    fn exprs2impl(&self,exprs:Vec<Expr>) -> Expr {
        let expr=self.fields()
            .struct_literal(self.struct_name(), exprs);
        let res=if self.is_try {
            parse_quote!{Ok(#expr)}
        } else {
            expr
        };
        res
    }

    pub fn implementation(&self, impl_fn:FnInfo) -> Expr {
        self.exprs2impl(match impl_fn {
            FnInfo::DefaultImpl(arity) => self.default_implementation(arity),
            FnInfo::ImplNullaryConst => self.nullary_const(),
            FnInfo::ImplUnaryConstArg(arg) => self.unary_const_arg(arg),
            FnInfo::ImplBinaryConstRhs(rhs) => self.binary_const_rhs(rhs),
        })
    }

    pub fn default_implementation(&self, arity:Arity) -> Vec<Expr> {
        match arity {
            Arity::Nullary => self.nullary_exprs(),
            Arity::Unary   => self.unary_exprs(),
            Arity::Binary  => self.binary_exprs(),
        }
    }
}