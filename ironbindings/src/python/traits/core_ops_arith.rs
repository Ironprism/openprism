use crate::{python::utils::ToSyn, utils::UnwrapAs};
use anyhow::Result;
use quote::ToTokens;
use rustdoc_types::{Crate, Id};
use std::collections::HashMap;

// add implace arith
const ARITH: &[(&str, char, &str, &str)] = &[
    ("core::ops::arith::Add", '+', "__add__", "__radd__"),
    ("core::ops::arith::Sub", '-', "__sub__", "__rsub__"),
    ("core::ops::arith::Mul", '*', "__mul__", "__rmul__"),
    ("core::ops::arith::Div", '/', "__truediv__", "__rtruediv__"),
    // floordiv
    ("core::ops::arith::Rem", '%', "__mod__", "__rmod__"),
    ("core::ops::arith::BitNot", '!', "__not__", "__rnot__"),
    ("core::ops::arith::BitAnd", '&', "__and__", "__rand__"),
    ("core::ops::arith::BitOr", '|', "__or__", "__ror__"),
    ("core::ops::arith::BitXor", '^', "__xor__", "__rxor__"),
    // Shr __lshift__
    // Shl __rshift__
    // pow?
    // __matmul__
    // neg
    // pos
    // abs
    // invert
    // clomplex
    // int
    // float
    // round
    // trunc
    // floor
    // ceil
];

pub fn handle(krate: &Crate, impls: &mut HashMap<String, Vec<Id>>) -> Result<Vec<syn::Item>> {
    let mut res = vec![];
    
    for (trait_path, op, method, rmethod) in ARITH {
        let method = syn::Ident::new(method, proc_macro2::Span::call_site());
        let rmethod = syn::Ident::new(rmethod, proc_macro2::Span::call_site());

        let impl_ids = impls.remove(*trait_path);
        if impl_ids.is_none() {
            continue;
        }
        let impl_ids = impl_ids.unwrap();
        // convert to a punct so we can use it in a quote
        let op = proc_macro2::Punct::new(*op, proc_macro2::Spacing::Alone);


        let mut dispatch: Vec<syn::Stmt> = vec![];

        for impl_id in impl_ids {
            let impl_item = krate.index.get(&impl_id).unwrap();
            let imp = impl_item.unwrap_as::<&rustdoc_types::Impl>();

            let trait_ = imp.trait_.as_ref().unwrap();
            let (args, _bindings) = match trait_.args.as_ref().unwrap().as_ref() {
                rustdoc_types::GenericArgs::AngleBracketed{args, bindings} => (args, bindings),
                _ => unreachable!(),
            };

            // hardcode the arith default generic
            if args.is_empty() {
                dispatch.push(syn::parse_quote!(
                    if let Ok(extracted) = other.extract(other) {
                        return Ok(self.0 #op extracted);
                    }
                ));
            } else {
                let add_type = match &args[0] {
                    rustdoc_types::GenericArg::Type(t) => t.to_syn().unwrap().to_token_stream(),
                    _ => unreachable!(),
                };
                dispatch.push(syn::parse_quote!(
                    if let Ok(extracted) = other.extract::<#add_type>(other) {
                        return Ok(self.0 #op extracted);
                    }
                ));
            };
        }
        res.push(syn::Item::Fn(syn::parse_quote!(
            #[automatically_derived]
            fn #method(&self, other: &pyo3::types::PyAny) -> PyResult<pyo3::types::PyAny> {
                #(#dispatch)*
                Err(pyo3::types::PyNotImplemented::get(other.py()))
            }
        )));
        res.push(syn::Item::Fn(syn::parse_quote!(
            #[automatically_derived]
            fn #rmethod(&self, other: &pyo3::types::PyAny) -> PyResult<pyo3::types::PyAny> {
                #(#dispatch)*
                Err(pyo3::types::PyNotImplemented::get(other.py()))
            }
        )));
    }
    Ok(res)
}
