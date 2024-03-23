use rustdoc_types::{Crate, Id};
use anyhow::Result;
use std::collections::HashMap;

/// here we ignore Ord and Eq as we don't have a direct translation for them
pub fn handle(_krate: &Crate, impls: &mut HashMap<String, Vec<Id>>) -> Result<Vec<syn::Item>> {
    impls.remove("core::cmp::Ord");
    impls.remove("core::cmp::Eq");

    // if the type has ord, just implement richcmp
    if impls.remove("core::cmp::PartialOrd").is_none() {
        return Ok(vec![syn::Item::Fn(syn::parse_quote!(
            #[automatically_derived]
            fn __richcmp__(
                &self,
                other: Self,
                op: pyo3::class::basic::CompareOp,
            ) -> bool {
                use pyo3::class::basic::CompareOp::*;
                match op {
                    Lt => self.0 < other.0,
                    Le => self.0 <= other.0,
                    Eq => self.0 == other.0,
                    Ne => self.0 != other.0,
                    Gt => self.0 > other.0,
                    Ge => self.0 >= other.0,
                }
            }
        ))]);
    }
    // otherwise impl eq if possible
    if impls.remove("core::cmp::PartialEq").is_none() {
        return Ok(vec![
            syn::Item::Fn(syn::parse_quote!(
                #[automatically_derived]
                fn __eq__(
                    &self,
                    other: Self,
                ) -> bool {
                    self.0 == other.0
                }
            )),
            syn::Item::Fn(syn::parse_quote!(
                #[automatically_derived]
                fn __ne__(
                    &self,
                    other: Self,
                ) -> bool {
                    self.0 != other.0
                }
            ))
        ]);
    };

    Ok(vec![])
}