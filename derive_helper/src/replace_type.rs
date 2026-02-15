use syn::{parse_quote, visit_mut::{self, VisitMut}, GenericParam, Generics, Ident, Type, TypePath, WhereClause, WherePredicate};

/// A replacer that replaces occurrences of `T` with `T2`.
pub struct ReplaceType {
    from: Ident,  // The type to replace
    to: TypePath, // The new type to replace with
}

impl ReplaceType {
    pub fn new(from:Ident, to:TypePath) -> Self {
        Self {from, to}
    }

    pub fn from_idents(from:&Ident, to:&Ident) -> Self {
        Self::new(from.clone(),parse_quote!{#to})
    }


    pub fn replace_type(& mut self, ty: & mut Type) {
        // Create the folder
        self.visit_type_mut(ty);
    }
}

impl VisitMut for ReplaceType {
    fn visit_type_path_mut(& mut self, ty: & mut TypePath) {
        if ty.path.is_ident(&self.from)  {
            *ty = self.to.clone()
        }
        visit_mut::visit_type_path_mut(self, ty);
    }
}


pub struct ReplaceTypes(Vec<ReplaceType>);


impl ReplaceTypes {
    pub fn from_into_iter(v:impl IntoIterator<Item=(Ident,Ident)>) -> Self {
        Self(v.into_iter()
              .map(|(from,to)|ReplaceType::from_idents(&from,&to))
              .collect())
    }

    pub fn from_generics_and_suffix(generics0:&Generics, generics:& mut Generics, suffix:&str) -> Self {
        let mut vs=Vec::new();
        let where_clause0=generics0.clone().make_where_clause().clone();
        for item in &generics0.params {
            if let GenericParam::Type(tp)=item {
                let ident=&tp.ident;
                let new_ident=Ident::new(&(ident.to_string()+suffix),ident.span());
                let mut tp_cloned=tp.clone();
                tp_cloned.ident=new_ident.clone();
                generics.params.push(GenericParam::Type(tp_cloned));
                vs.push((ident.clone(), new_ident))
            }
        }
        let mut s=Self::from_into_iter(vs);
        let where_clause=generics.make_where_clause();
        s.extend_where_clause(&where_clause0, where_clause);
        s
    }

    pub fn extend_where_clause(&mut self, where_clause0:&WhereClause, where_clause:& mut WhereClause) {
        let preds=& mut where_clause.predicates;
        let preds0=where_clause0.predicates.clone();
        for pred in preds0.iter() {
            if let WherePredicate::Type(pty)=pred {
                for rt in & mut self.0 {
                    let mut pty_new=pty.clone();
                    rt.replace_type(& mut pty_new.bounded_ty);
                    preds.push(WherePredicate::Type(pty_new));
                }
            }
        }
    }

    pub fn gen_type(& mut self, ty: &Type) -> Type {
        let mut tyc=ty.clone();
        for rt in & mut self.0 {
            rt.replace_type(& mut tyc);
        }
        tyc
    }
}