use rustdoc_types::{Crate, Id};
use anyhow::Result;
use std::collections::HashMap;

pub fn handle(_krate: &Crate, impls: &mut HashMap<String, Vec<Id>>) -> Result<Vec<syn::Item>> {
    if impls.remove("core::clone::Clone").is_none() {
        return Ok(vec![]);
    }
    
    Ok(vec![syn::Item::Fn(syn::parse_quote!(
        #[automatically_derived]
        fn __deepcopy__(&self, memo: pyo3::types::PyDict) -> PyResult<Self> {
            Ok(Self(self.0.clone()))
        }
    ))])
}