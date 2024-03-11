use anyhow::Result;
use crate::parse::*;
use crate::utils::{Twine, TwinePush};
use super::utils::{ToIdent, ToSyn};
use super::getattr::gen_getattr;

pub fn parse_method(func: &Function) -> Result<syn::Item> {
    todo!();
}

pub fn parse_struct(ztruct: &Struct) -> Result<Twine<syn::Item>> {
    let mut res = Twine::new();
    // TODO!: actually handle varians, this impl just handle one
    assert!(ztruct.variants.len() == 1, "Only one variant per struct is supported for now");
    for variant in ztruct.variants.iter() {
        let mut method_names = Vec::new();
        let mut methods = Vec::new();

        // define the struct
        let ztruct_name = &ztruct.name;
        let ztruct_id = ztruct.id.to_ident();
        let ztruct_path = ztruct.path.join("::");
        let ztruct_module = ztruct.path[0..ztruct.path.len().saturating_sub(1)].join(".");
        res.push(syn::Item::Struct(syn::parse_quote!(
            #[pyclass(name = #ztruct_name, module = #ztruct_module)]
            pub struct #ztruct_id(#ztruct_path);
        )));
    
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
                    "std::cmp::PartialOrd" // not Ord so we avoid implemeting twice
                    | "core::cmp::PartialOrd" => {
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
                methods.push(parse_method(func)?);
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
    }

    Ok(res)
}