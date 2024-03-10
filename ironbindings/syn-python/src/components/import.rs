//! Module defining the import statement for Python projects.
use super::component::Component;

pub struct Import {
    name: String,
    alias: Option<String>,
}

impl ToString for Import {
    fn to_string(&self) -> String {
        match &self.alias {
            Some(alias) => format!("import {} as {}", self.name, alias),
            None => format!("import {}", self.name),
        }
    }
}

impl Component for Import {}
