use anyhow::{Context, Result};

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
    fn to_syn(&self) -> Result<Self::Output>;
}

impl ToSyn for rustdoc_types::Type {
    type Output = syn::Type;
    fn to_syn(&self) -> Result<Self::Output> {
        match self {
            rustdoc_types::Type::Primitive(p) => syn::parse_str::<syn::Type>(p)
                .with_context(|| format!("Could not parse type {} as primitive", p)),
            rustdoc_types::Type::Generic(generic) => syn::parse_str(generic)
                .with_context(|| format!("Could not parse type {} as generic", generic)),
            rustdoc_types::Type::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => {
                let lifetime = match lifetime.as_ref() {
                    Some(l) => Some(
                        syn::parse_str::<syn::Lifetime>(l)
                            .with_context(|| format!("Could not parse as lifetime {}", l))?,
                    ),
                    None => None,
                };
                let mutable = if *mutable {
                    Some(quote::quote!(mut))
                } else {
                    None
                };
                let type_ = type_.to_syn()?;
                Ok(syn::parse_quote!(& #lifetime #mutable #type_))
            }
            rustdoc_types::Type::ResolvedPath(path) => Ok(syn::parse_str(&path.name)
                .with_context(|| format!("Could not parse as path {}", path.name))?),
            _ => panic!("Cannot convert {:?} to syn::Type", self),
        }
    }
}
