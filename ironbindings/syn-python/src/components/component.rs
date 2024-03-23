use std::fmt::Display;

/// A component of a Python project.
///
/// # Implementation details
/// A python component must be able to be converted to a string
/// so that it can be written to a file.
pub trait Component: Display {}