use crate::utils::UnwrapAs;
use crate::python::utils::{ToIdent, ToSyn};
use anyhow::Result;
use rustdoc_types::*;

pub fn parse_function(krate: &Crate, func_id: &Id) -> Result<syn::Item> {
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
        arg_names.push(arg_name);
    }

    let func_path = krate.paths.get(func_id).unwrap().path.join("::");
    let func_path = syn::parse_str::<syn::Path>(&func_path)?;
    let stmt = quote::quote!(
        #func_path(#(#arg_names),*)
    );

    Ok(syn::parse_quote!(
        #[pyfunction]
        #[pyo3(name = #func_name)]
        #[automatically_derived]
        pub fn #func_ident(#(#args),*) #ret {
            #stmt
        }
    ))
}
