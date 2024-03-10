//! Module defining the decorator component for Python projects.
use super::component::Component;

pub struct Decorator {
    name: String,
}

impl ToString for Decorator {
    fn to_string(&self) -> String {
        format!("@{}", self.name)
    }
}

impl Component for Decorator {}