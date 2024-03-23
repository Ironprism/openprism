use std::fmt::Display;

use serde::Serialize;

/// A component of a Python project.
///
/// # Implementation details
/// A python component must be able to be converted to a string
/// so that it can be written to a file.
pub trait Component: Display + Clone + Serialize + PartialEq {}
