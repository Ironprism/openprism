use anyhow::Result;
use std::collections::HashMap;
use rustdoc_types::{Crate, Id};

pub mod core_clone_clone;
pub mod core_fmt_debug;
pub mod core_fmt_display;
pub mod core_hash_hash;
pub mod core_ops_arith;
pub mod cmp_and_eq;

// TODO!:
// index -> get or getitem
// call
// iter, next
// len
// bool

pub fn known_but_unhandled(_krate: &Crate, impls: &mut HashMap<String, Vec<Id>>) -> Result<Vec<syn::Item>> {
    // remove marker traits
    impls.remove("core::marker::Send");
    impls.remove("core::marker::Sync");

    // stuff not useful for python
    impls.remove("core::any::Any");
    impls.remove("core::marker::Freeze");
    impls.remove("core::marker::Unpin");
    impls.remove("core::panic::unwind_safe::UnwindSafe");
    impls.remove("core::panic::unwind_safe::RefUnwindSafe");
    impls.remove("core::marker::StructuralPartialEq");
    impls.remove("core::borrow::Borrow");
    impls.remove("alloc::borrow::ToOwned");

    // remove traits with generics
    impls.remove("core::borrow::BorrowMut");

    Ok(vec![])
}

pub const TRAITS_HANDLERS: &[fn(&Crate, &mut HashMap<String, Vec<Id>>) -> Result<Vec<syn::Item>>] = &[
    core_fmt_debug::handle,
    core_fmt_display::handle,
    core_hash_hash::handle,
    cmp_and_eq::handle,
    core_clone_clone::handle,
    core_ops_arith::handle,
    known_but_unhandled,
];
