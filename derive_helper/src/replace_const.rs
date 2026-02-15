use syn::{parse_quote, visit_mut::{self, VisitMut}, DeriveInput, GenericParam, Ident, Type};


pub fn find_const(input:&DeriveInput) -> Option<Ident> {
    for p in &input.generics.params {
        if let GenericParam::Const(c) = p {
            if c.ty == parse_quote!{usize} {
                return Some(c.ident.clone());
            }
        }
    }
    None
}


/// A replacer that replaces occurrences of `T` with `T2`.
pub struct ReplaceConst {
    from: Ident,  // The type to replace
    to: Ident, // The new type to replace with
}

impl ReplaceConst {
    pub fn new(from:Ident, to:Ident) -> Self {
        Self {from, to}
    }

    pub fn replace_const(& mut self, ty: &Type) -> Type {
        let mut ty_res=ty.clone();
        self.visit_type_mut(& mut ty_res);
        ty_res
    }
}

impl VisitMut for ReplaceConst {
    fn visit_ident_mut(& mut self, i: & mut Ident) {
        if *i == self.from  {
            *i = self.to.clone()
        }
        visit_mut::visit_ident_mut(self, i);
    }
}