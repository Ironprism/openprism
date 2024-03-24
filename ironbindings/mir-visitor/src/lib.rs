//! Crate providing traits to visit the MIR (Mid-level Intermediate Representation) of a Rust program.
#![feature(rustc_private)]
extern crate rustc_middle;
pub mod visitors;