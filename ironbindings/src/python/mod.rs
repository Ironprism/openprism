use anyhow::Result;
use crate::parse::*;
use crate::utils::{Twine, TwinePush};

trait ToIdent {
    fn to_ident(&self) -> syn::Ident;
}

impl ToIdent for rustdoc_types::Id {
    fn to_ident(&self) -> syn::Ident {
        quote::format_ident!("ID_{}", self.0.replace(":", "_"))
    }
}

fn parse_method(func: &Function) -> Result<syn::Item> {
    todo!();
}

fn gen_getattr(names: &[String], method: bool) -> Result<syn::ItemFn> {
    let attr = if method {
        quote::quote!()
    } else {
        quote::quote!(#[pyfunction])
    };
    
    if names.is_empty() {
        return Ok(syn::parse_quote!(
            #attr
            pub fn __getattr__(&self, name: &str) -> PyResult<()> {
                Err(PyAttributeError::new_err(format!(
                    "The `{{}}` does not exists.",
                    name
                )))
            }
        ));
    }


    let crate::tfidf::TFIDF{
        unique_terms_list,
        tfidf,
    } = crate::tfidf::tfidf_gen(names)?;

    let mut tfidfs_rows = Vec::new();
    for row in tfidf.iter() {
        let vals = row.iter().map(|(name, score)| quote::quote!((#name, #score)));
        tfidfs_rows.push(quote::quote!(&[ #(#vals),* ]));
    }

    Ok(syn::parse_quote!(
        #attr
        pub fn __getattr__(&self, name: &str) -> PyResult<()> {
            const NAMES: &[&str] = &[#(#names),*];
            const TERMS: &[&str] = &[#(#unique_terms_list),*];
            const TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[#(#tfidfs_rows),*];

            Err(PyAttributeError::new_err(format!(
                "The `{{}}` does not exists, did you mean one of the following?\n\n{{}}",
                name,
                tfidf_splitter(name, TERMS, NAMES, TFIDF_FREQUENCIES)
                    .map(|method| {{
                        format!("* `{}`",  method )
                    }})
                    .take(10)
                    .collect::<Vec<String>>()
                    .join("\n"),
            )))
        }
    ))
}

fn parse_struct(ztruct: &Struct) -> Result<Twine<syn::Item>> {
    let ztruct_name = &ztruct.name;
    let ztruct_id = ztruct.id.to_ident();
    let ztruct_path = syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: ztruct
                .path
                .iter()
                .map(|s| syn::PathSegment {
                    ident: syn::Ident::new(s, proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                })
                .collect(),
        },
    });
    let mut res = Twine::new();
    res.push(syn::Item::Struct(syn::parse_quote!(
        #[pyclass(name = #ztruct_name)]
        pub struct #ztruct_id(#ztruct_path);
    )));

    let mut method_names = Vec::new();
    let mut methods = Vec::new();
    let mut ord_implemented = false;

    for variant in ztruct.variants.iter() {
        for imp in variant.impls.iter() {
            // handle special traits
            if let Some(trait_) = &imp.trait_ {
                match trait_.path.join("::").as_str() {
                    "std::fmt::Debug" | "core::fmt::Debug" => {
                        methods.push(syn::Item::Fn(syn::parse_quote!(
                            fn __repr__(&self) -> PyResult<String> {
                                Ok(format!("{:?}", self.0))
                            }
                        )));
                        continue;
                    }
                    "std::hash::Hash" | "core::hash::Hash" => {
                        methods.push(syn::Item::Fn(syn::parse_quote!(
                            fn __hash__(&self) -> PyResult<isize> {
                                use core::hash::{Hash, Hasher};
                                use std::collections::hash_map::DefaultHasher;
                                let mut hasher = DefaultHasher::new();
                                self.0.hash(&mut hasher);
                                Ok(hasher.finish() as isize)
                            }
                        )));
                        continue;
                    }
                    "std::cmp::Ord"
                    | "core::cmp::Ord"
                    | "std::cmp::PartialOrd"
                    | "core::cmp::PartialOrd" => {
                        if ord_implemented {
                            continue;
                        }
                        ord_implemented = true;
                        methods.push(syn::Item::Fn(syn::parse_quote!(
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
                        )));
                        continue;
                    }
                    _ => {}
                }
            }
            // or just get the method names
            for func in imp.functions.iter() {
                method_names.push(func.name.clone());
            }
        }
    }

    methods.push(syn::Item::Fn(gen_getattr(method_names.as_slice(), true)?));

    res.push(syn::Item::Impl(syn::parse_quote!(
        #[pymethods]
        impl #ztruct_id {
            #(
                #methods
            )*
        }
    )));

    //for variant in ztruct.variants.iter() {
    //    for imp in variant.impls.iter() {
    //        res.push(parse_impl(&ztruct.id, imp));
    //    }
    //}

    Ok(res)
}

fn parse_function(func: &Function) -> Result<syn::Item> {
    let func_name = &func.name;
    let func_id = func.id.to_ident();

    todo!();
    /*
    syn::parse_quote!(
        #[pyfunction]
        #[pyo3(name = #func_name)]
        pub fn #func_id() #ret {
            #stmt
        }
    ) */
}

fn parse_module(module: &Module) -> Result<Twine<syn::Item>> {
    let mut items = Twine::new();
    let mut names = Vec::new();
    let mut getattr_exists = false;
    for module in module.modules.iter() {
        items.push(parse_module(module)?);
        names.push(module.name.clone());
    }
    for function in module.functions.iter() {
        //items.push(parse_function(function)?);
        names.push(function.name.clone());
        // allow the user to define its own getattr
        if function.name == "__getattr__" {
            getattr_exists = true;
        }
    }
    for ztruct in module.structs.iter() {
        items.push(parse_struct(ztruct)?);
        names.push(ztruct.name.clone());
    }

    if getattr_exists {
        items.push(syn::Item::Fn(gen_getattr(&names, false)?));
    }

    Ok(items)
}

pub fn python_bindgen(module: &Module) -> Result<syn::File> {
    let mut items: Twine<syn::Item> = Twine::new();

    items.push(syn::Item::Use(syn::parse_quote!(
        use pyo3::prelude::*;
    )));

    // functions needed for the __getattr__ tfidf
    items.push(syn::Item::Fn(crate::tfidf::get_tfidf_splitter()));
    items.push(syn::Item::Fn(crate::tfidf::get_tfidf_matcher()));

    items.push(parse_module(module)?);

    Ok(syn::File {
        shebang: None,
        attrs: vec![],
        items: items.to_vec(),
    })
}
