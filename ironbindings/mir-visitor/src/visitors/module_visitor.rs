//! Submodule providing the trait defining a MIR visitor specifically for modules.
//! 


use rustc_middle::mir::visit::Visitor;

use super::{function_visitor::FunctionVisitable, struct_visitor::StructVisitable};

pub trait ModuleVisitable {
    type Struct: StructVisitable;
    type Function: FunctionVisitable;

    fn from_name(name: String) -> Self;
    fn set_module_docstring(&self, docstring: String);
    fn add_submodule(&self, submodule: Self);
    fn add_struct(&self, struct_: Self::Struct);
    fn add_function(&self, function: Self::Function);
}

pub struct ModuleVisitor {

}

impl Visitor<'_> for ModuleVisitor {

}