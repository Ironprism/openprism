use super::getattr::gen_getattr;
use super::utils::{ToIdent, ToSyn};
use crate::utils::{Twine, TwinePush, UnwrapAs};
use anyhow::{Result, ensure};
use std::collections::HashMap;
use crate::python::traits::TRAITS_HANDLERS;
use rustdoc_types::*;

pub fn parse_method(krate: &Crate, func_id: &Id, trait_: Option<&Path>) -> Result<syn::Item> {
    let func_item = krate.index.get(func_id).unwrap();
    let func = func_item.unwrap_as::<&Function>();

    let func_name = func_item.name.as_ref().unwrap();
    let func_ident = func_id.to_ident();

    let ret = if let Some(ret) = &func.decl.output {
        let ret = ret.to_syn()?;
        quote::quote!(-> #ret)
    } else {
        quote::quote!()
    };

    let mut args = Vec::new();
    let mut arg_names = Vec::new();
    for (arg_name, arg_type) in func.decl.inputs.iter() {
        let arg_name = syn::Ident::new(arg_name, proc_macro2::Span::call_site());
        let arg_type = arg_type.to_syn()?;
        args.push(quote::quote!(#arg_name: #arg_type));

        if arg_name != "self" {
            arg_names.push(arg_name);
        }
    }

    let stmt = if let Some(trait_path) = trait_ {
        let trait_path = krate.paths.get(&trait_path.id).unwrap().path.join("::");
        let trait_path = syn::parse_str::<syn::Path>(&trait_path)?;
        quote::quote!(
            <Self as #trait_path>::#func_ident(self.0, #(#arg_names),*)
        )
    } else {
        quote::quote!(
            self.0.#func_ident(#(#arg_names),*)
        )
    };

    Ok(syn::parse_quote!(
        #[pyo3(name = #func_name)]
        #[automatically_derived]
        pub fn #func_ident(#(#args),*) #ret {
            #stmt
        }
    ))
}

pub fn parse_struct(krate: &Crate, ztruct_id: &Id) -> Result<Twine<syn::Item>> {
    let mut res = Twine::new();
    let ztruct_item = krate.index.get(ztruct_id).unwrap();
    let ztruct = ztruct_item.unwrap_as::<&Struct>();
    
    ensure!(ztruct.generics.params.is_empty(), "Generics are not supported yet");

    // define the struct
    let ztruct_name = ztruct_item.name.as_ref().unwrap();
    let ztruct_ident = ztruct_id.to_ident();
    let path = &krate.paths.get(ztruct_id).unwrap().path;
    let ztruct_path = path.join("::");
    let ztruct_path = syn::parse_str::<syn::Path>(&ztruct_path)?;
    let ztruct_module = path[0..path.len().saturating_sub(1)].join(".");
    res.push(syn::Item::Struct(syn::parse_quote!(
        #[pyclass(name = #ztruct_name, module = #ztruct_module)]
        #[automatically_derived]
        pub struct #ztruct_ident(#ztruct_path);
    )));

    // create a map of all the impls that regard traits so the handlers can
    // in O(1) check if they can handle it
    // we cannot directly iter over the impls becasue traits like
    // PartialEq and PartialOrd have mutually esclusive impls so
    // this way we can handle logics with arbitrary complexity  
    let mut trait_impls = HashMap::new();
    let mut impls = Vec::new();

    for impl_id in &ztruct.impls {
        let impl_item = krate.index.get(impl_id).unwrap();
        let imp = impl_item.unwrap_as::<&Impl>();

        match &imp.trait_ {
            Some(trait_) => {
                let trait_path = krate.paths.get(&trait_.id).unwrap().path.join("::");
                let entry = trait_impls.entry(trait_path).or_insert_with(Vec::new);
                entry.push(impl_id.clone());
            },
            None => {
                impls.push(imp.clone());
            }
        }
    }

    let mut method_names = Vec::new();
    let mut methods = Vec::new();
    // handle what can be handled
    for handler in TRAITS_HANDLERS {
        let handled = handler(krate, &mut trait_impls)?;
        methods.extend(handled);
    }

    if !trait_impls.is_empty() {
        log::warn!("Unimplemented traits: {:?}", trait_impls.keys().collect::<Vec<_>>());
    }

    for imp in &impls {
        // handle special traits
        // or just get the method names
        for func_id in &imp.items {
            let func_item = krate.index.get(func_id).unwrap();
            let func = func_item.unwrap_as::<&Function>();
            let func_name = func_item.name.as_ref().unwrap();

            if func.generics.params.len() > 0 {
                log::warn!(
                    "Skipping method {} with generics, not supported yet",
                    func_name
                );
                continue;
            }

            method_names.push(func_name.clone());
            methods.push(parse_method(krate, func_id, imp.trait_.as_ref())?);
        }
    }

    methods.push(syn::Item::Fn(gen_getattr(method_names.as_slice(), true)?));

    res.push(syn::Item::Impl(syn::parse_quote!(
        #[pymethods]
        impl #ztruct_ident {
            #(
                #methods
            )*
        }
    )));

    Ok(res)
}
