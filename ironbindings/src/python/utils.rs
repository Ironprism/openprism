
/// hack to add a method to rustdoc_types::Id to convert it to a syn::Ident
pub trait ToIdent {
    fn to_ident(&self) -> syn::Ident;
}

impl ToIdent for rustdoc_types::Id {
    fn to_ident(&self) -> syn::Ident {
        quote::format_ident!("ID_{}", self.0.replace(":", "_"))
    }
}

/// generic adapter from rustdoc_types to syn,
/// the main goal is to write rustdoc_types::Type to syn::Type
pub trait ToSyn {
    type Output;
    fn to_syn(&self) -> Self::Output;
}

impl ToSyn for rustdoc_types::Type {
    type Output = syn::Type;
    fn to_syn(&self) -> Self::Output {
        match self {
            _ => todo!(),
        }
    }
}
