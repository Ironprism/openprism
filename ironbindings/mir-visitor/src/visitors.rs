//! Submodule providing visitors for the different types of MIR nodes.
//! 
//! All visitors are defined as traits, and we implement the trait Visitor from the crate rustc_middle
//! for each of them with a blanket implementation. 
pub mod struct_visitor;
pub mod module_visitor;
pub mod function_visitor;