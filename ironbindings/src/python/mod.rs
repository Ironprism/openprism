use crate::utils::{Twine, TwinePush, UnwrapAs};
use crate::python::utils::ToIdent;
use anyhow::Result;
pub mod getattr;
pub mod utils;
mod ztruct;
use ztruct::*;
mod function;
use function::*;
pub mod traits;
use rustdoc_types::*;

/// Internal method
pub fn parse_module(krate: &Crate, module_id: &Id) -> Result<Twine<syn::Item>> {
    let mut result = Twine::new();
    // name of the things in this module (needed for TFIDF getattr)
    let mut names = Vec::new();
    // the user might define its own getattr so we need to check if it exists
    let mut getattr_exists = false;
    // the code to put inside the module registration to make visible all
    // sub items
    let mut registrations = Vec::new();

    let module_item = krate.index.get(module_id).unwrap();
    let module = module_item.unwrap_as::<&Module>();

    for item_id in &module.items {
        let item = krate.index.get(&item_id).unwrap();
        match &item.inner {
            ItemEnum::Module(_) => {
                let module_name = item.name.as_ref().unwrap();
                result.push(parse_module(krate, item_id)?);
                names.push(module_name.clone());
        
                // generate the code to register this module on the parent module
                let module_name = item.name.as_ref().unwrap();
                let module_id = item.id.to_ident();
                let module_registration = quote::format_ident!("register_{}", module_id);
                registrations.push(quote::quote!(
                    let #module_id = PyModule::new_bound(module.py(), stringhify!(#module_name));
                    #module_registration(#module_id)?
                    module.add_submodule(#module_id)?;
                ));
            }
            ItemEnum::Function(_) => {
                let func_name = item.name.as_ref().unwrap();
                result.push(parse_function(krate, item_id)?);
                names.push(func_name.clone());
                // allow the user to define its own getattr
                if func_name == "__getattr__" {
                    getattr_exists = true;
                }
                // generate the code to register this function on the parent module
                let function_id = item_id.to_ident();
                registrations.push(quote::quote!(
                    m.add_function(pyo3::wrap_pyfunction!(#function_id))?;
                ));
            }
            ItemEnum::Struct(_) => {
                let struct_name = item.name.as_ref().unwrap();
                result.push(parse_struct(krate, item_id)?);
                names.push(struct_name.clone());
                // generate the code to register this struct on the parent module
                let ztruct_id = item_id.to_ident();
                registrations.push(quote::quote!(
                    m.add_class::<#ztruct_id>()?;
                ));
            }
            _ => todo!("{:?}", item.inner),
        }
    }

    // if not defined, add the default tfidf getattr
    if !getattr_exists {
        result.push(syn::Item::Fn(getattr::gen_getattr(&names, false)?));
    }

    // register all the sub items
    let module_registration = quote::format_ident!("register_{}", module_id.to_ident());
    result.push(syn::Item::Fn(syn::parse_quote!(
        #[automatically_derived]
        fn #module_registration(module: &Bound<'_, PyModule>) -> PyResult<()> {
            #(#registrations)*
            Ok(())
        }
    )));

    Ok(result)
}

/// Main entrypoint for the python bindings
pub fn python_bindgen(krate: &Crate) -> Result<syn::File> {

    let mut items: Twine<syn::Item> = Twine::new();

    // imports
    items.push(syn::Item::Use(syn::parse_quote!(
        use pyo3::prelude::*;
    )));

    // create the module entrypoint
    let root_item = krate.index.get(&krate.root)
        .expect("No root module in rustdoc??");
    let module_name = &root_item.name;
    let module_id = root_item.id.to_ident();
    let module_registration = quote::format_ident!("register_{}", module_id);
    items.push(syn::Item::Fn(syn::parse_quote!(
        #[pymodule]
        #[pyo3(name = #module_name)]
        #[automatically_derived]
        fn #module_id(m: &Bound<'_, PyModule>) -> PyResult<()> {
            #module_registration(m)?;
            Ok(())
        }
    )));

    // functions needed for the __getattr__ tfidf
    items.push(syn::Item::Fn(crate::tfidf::get_tfidf_splitter()));
    items.push(syn::Item::Fn(crate::tfidf::get_tfidf_matcher()));

    // run the recursive parsing
    items.push(parse_module(krate, &krate.root)?);

    Ok(syn::File {
        shebang: None,
        attrs: vec![],
        items: items.to_vec(),
    })
}
