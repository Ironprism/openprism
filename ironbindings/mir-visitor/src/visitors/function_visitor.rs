//! Submodule providing the trait defining a MIR visitor specifically for functions (not methods).
//! 
//! Functions differ from methods in that they are not associated with a struct or an enum, and
//! they are not called on an instance of a struct or an enum.

pub trait FunctionVisitable {
    fn from_name(name: String) -> Self;
}