//! Submodule providing the trait defining a MIR visitor specifically for structs.
//! 
//! The trait StructVisitor implements the Visitor trait, and we expect this specific
//! visitor to be used to visit structs in the MIR.

pub trait StructVisitable {
    fn from_name(name: String) -> Self;
}
