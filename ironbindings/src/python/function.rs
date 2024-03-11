use anyhow::Result;
use crate::parse::*;
use super::utils::{ToIdent, ToSyn};

pub fn parse_function(func: &Function) -> Result<syn::Item> {
    let func_name = &func.name;
    let func_id = func.id.to_ident();

    let ret = if let Some(ret) = &func.func.decl.output {
        let ret = ret.to_syn();
        quote::quote!(-> #ret)
    } else {
        quote::quote!()
    };

    let stmt = quote::quote!(); //TODO!:

    Ok(syn::parse_quote!(
        #[pyfunction]
        #[pyo3(name = #func_name)]
        pub fn #func_id() #ret {
            #stmt
        }
    ))
}