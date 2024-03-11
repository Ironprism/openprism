use anyhow::Result;
use crate::parse::*;
use crate::utils::{Twine, TwinePush};
pub mod getattr;
pub mod utils;
use utils::ToIdent;
mod ztruct;
use ztruct::*;
mod function;
use function::*;


/// Internal method
pub fn parse_module(module: &Module) -> Result<Twine<syn::Item>> {
    let mut result = Twine::new();
    // name of the things in this module (needed for TFIDF getattr)
    let mut names = Vec::new();
    // the user might define its own getattr so we need to check if it exists
    let mut getattr_exists = false;
    // the code to put inside the module registration to make visible all
    // sub items
    let mut registrations = Vec::new();
    for module in module.modules.iter() {
        result.push(parse_module(module)?);
        names.push(module.name.clone());

        // generate the code to register this module on the parent module
        let module_name = &module.name;
        let module_id = module.id.to_ident();
        let module_registration = quote::format_ident!("register_{}", module_id);
        registrations.push(quote::quote!(
            let #module_id = PyModule::new_bound(module.py(), stringhify!(#module_name));
            #module_registration(#module_id)?
            module.add_submodule(#module_id)?;
        ));

    }
    for function in module.functions.iter() {
        result.push(parse_function(function)?);
        names.push(function.name.clone());
        // allow the user to define its own getattr
        if function.name == "__getattr__" {
            getattr_exists = true;
        }
        // generate the code to register this function on the parent module
        let function_id = function.id.to_ident();
        registrations.push(quote::quote!(
            m.add_function(pyo3::wrap_pyfunction!(#function_id))?;
        ));
    }
    for ztruct in module.structs.iter() {
        result.push(parse_struct(ztruct)?);
        names.push(ztruct.name.clone());
        // generate the code to register this struct on the parent module
        let ztruct_id = ztruct.id.to_ident();
        registrations.push(quote::quote!(
            m.add_class::<#ztruct_id>()?;
        ));
    }

    // if not defined, add the default tfidf getattr
    if !getattr_exists {
        result.push(syn::Item::Fn(getattr::gen_getattr(&names, false)?));
    }

    // register all the sub items
    let module_registration = quote::format_ident!("register_{}", module.id.to_ident());
    result.push(syn::Item::Fn(syn::parse_quote!(
        fn #module_registration(module: &Bound<'_, PyModule>) -> PyResult<()> {
            #(#registrations)*
            Ok(())
        }
    )));

    Ok(result)
}

/// Main entrypoint for the python bindings
pub fn python_bindgen(module: &Module) -> Result<syn::File> {
    let mut items: Twine<syn::Item> = Twine::new();

    // imports
    items.push(syn::Item::Use(syn::parse_quote!(
        use pyo3::prelude::*;
    )));

    // create the module entrypoint
    let module_name = &module.name;
    let module_id = module.id.to_ident();
    let module_registration = quote::format_ident!("register_{}", module_id);
    items.push(syn::Item::Fn(syn::parse_quote!(
        #[pymodule]
        #[pyo3(name = #module_name)]
        fn #module_id(m: &Bound<'_, PyModule>) -> PyResult<()> {
            #module_registration(m)?;
            Ok(())
        }
    )));

    // functions needed for the __getattr__ tfidf
    items.push(syn::Item::Fn(crate::tfidf::get_tfidf_splitter()));
    items.push(syn::Item::Fn(crate::tfidf::get_tfidf_matcher()));

    // run the recursive parsing
    items.push(parse_module(module)?);

    Ok(syn::File {
        shebang: None,
        attrs: vec![],
        items: items.to_vec(),
    })
}
