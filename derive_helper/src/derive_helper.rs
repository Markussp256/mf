use syn::{punctuated::Punctuated, token::Comma, DeriveInput, Expr, Ident, Path, Type, WherePredicate};
use crate::{implementor::Implementor, preprocessor::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arity  {
    Nullary,
    Unary,
    Binary
}
pub use Arity::*;

pub enum FnInfo<'a> {
    DefaultImpl(Arity),
    ImplNullaryConst,
    ImplUnaryConstArg(&'a Expr),
    ImplBinaryConstRhs(&'a Expr),
}


pub mod subfields {
    use syn::{parse_quote, DeriveInput, Expr};
    use crate::fields_trait::Fields;
    use crate::preprocessor::Preprocessor;

    pub fn subfields(input:& mut DeriveInput,expr:Expr) -> Vec<Expr> {
        Preprocessor::new(input)
            .fields()
            .subs(expr)
    }

    pub fn self_subfields(input:& mut DeriveInput) -> Vec<Expr> {
        subfields(input, parse_quote!{self})
    }

    pub fn rhs_subfields(input:& mut DeriveInput) -> Vec<Expr> {
        subfields(input, parse_quote!{rhs})
    }
}
pub use subfields::*;



type Out<'a,const N:usize>=(proc_macro2::TokenStream, Punctuated<WherePredicate, Comma>,[(Type, Vec<Type>);N], Expr);

pub struct DeriveHelper<'a> {
    input: &'a mut DeriveInput,
    tr:&'a Path,
    fn_name:&'a Ident,
    add_gen_types:Vec<&'static str>,
    add_const_gen:Vec<&'static str>
}

impl<'a> DeriveHelper<'a> {
    pub fn new(input: &'a mut DeriveInput, tr:&'a Path, fn_name:&'a Ident) -> Self {
        Self{input, tr, fn_name, add_gen_types:Vec::new(),add_const_gen:Vec::new()}
    }

    pub fn add_gen_types(self,add_gen_types:Vec<&'static str>) -> Self {
        let mut s=self;
        s.add_gen_types.extend(add_gen_types);
        s
    }


    pub fn add_const_gen(self, add_const_gen:Vec<&'static str>) -> Self {
        let mut s=self;
        s.add_const_gen.extend(add_const_gen);
        s
    }

    pub fn preprocess(self, arity:Arity) -> Out<'a,1> {
        self.create_output::<1>(false, FnInfo::DefaultImpl(arity))
    }

    pub fn for_closed_try(self) -> Out<'a,1> {
        self.create_output::<1>(true,FnInfo::DefaultImpl(Binary))
    }

    pub fn for_try(self) -> Out<'a,3> {
        self.create_output::<3>(true, FnInfo::DefaultImpl(Binary))
    }

    pub fn nullary_const(self) -> Out<'a,1> {
        self.create_output::<1>(false, FnInfo::ImplNullaryConst)
    }

    pub fn unary_const_arg(self, is_try:bool, arg:&'a Expr) -> Out<'a,1> {
        self.create_output::<1>(is_try,FnInfo::ImplUnaryConstArg(arg))
    }

    pub fn binary_const_rhs<const N:usize>(self, is_try:bool, rhs:&'a Expr) -> Out<'a,N> {
        self.create_output::<N>(is_try,FnInfo::ImplBinaryConstRhs(rhs))
    }

    pub fn extended1(self) -> Out<'a, 2> {
        self.create_output::<2>(false,FnInfo::DefaultImpl(Unary))
    }

    pub fn extended2(self) -> Out<'a, 3> {
        self.create_output::<3>(false,FnInfo::DefaultImpl(Binary))
    }

    fn create_output<const N:usize>(self,is_try:bool, fn_info:FnInfo) -> Out<N> {
        let (igen, wc, tys)=preprocess_no_impl_add_gen_const_types(self.input,self.add_gen_types,self.add_const_gen);
        let implementation=Implementor::new(self.input,self.tr,self.fn_name,is_try)
                .implementation(fn_info);
        (igen, wc, tys, implementation)
    }
}
