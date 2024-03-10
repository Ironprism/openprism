//! Module defining the function component for Python projects.
use super::component::Component;
use super::typing::Typing;
use super::docstring::Docstring;
use super::decorator::Decorator;

pub struct Function {
    name: String,
    docstring: Option<Docstring>,
    corpus: String,
    decorators: Vec<Decorator>,
}