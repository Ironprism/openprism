//! Submodule defining a Python module.

use super::class::Class;
use super::component::Component;
use super::function::Function;
use super::import::{Import, ImportFrom};
use crate::python_token::Token;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleDocstring {
    /// The docstring of the module.
    docstring: String,
}

impl Display for ModuleDocstring {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\"\"\"\n{}\n\"\"\"", self.docstring)
    }
}

impl Component for ModuleDocstring {}

impl ModuleDocstring {
    pub fn new(docstring: String) -> Result<ModuleDocstring, String> {
        if docstring.is_empty() {
            return Err("The module docstring cannot be empty.".to_string());
        }

        if docstring.contains("\"\"\"") {
            return Err("The module docstring cannot contain triple quotes.".to_string());
        }

        Ok(ModuleDocstring { docstring })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleType {
    Init,
    Submodule(Token),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Module {
    /// The name of the Python file that will be generated.
    module_type: ModuleType,
    /// The docstring of the module.
    docstring: Option<ModuleDocstring>,
    /// The imports of the module, of the form `import x.y.z as a`.
    imports: Vec<Import>,
    /// The imports of the module, of the form `from x.y.z import a as b`.
    import_froms: Vec<ImportFrom>,
    /// The functions in the module.
    functions: Vec<Function>,
    /// The classes in the module.
    classes: Vec<Class>,
}

impl Module {
    pub fn new(module_type: ModuleType) -> Module {
        Module {
            module_type,
            docstring: None,
            imports: Vec::new(),
            import_froms: Vec::new(),
            functions: Vec::new(),
            classes: Vec::new(),
        }
    }

    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }

    pub fn add_import_from(&mut self, import_from: ImportFrom) {
        self.import_froms.push(import_from);
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    pub fn set_docstring(&mut self, docstring: ModuleDocstring) {
        self.docstring = Some(docstring);
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(docstring) = &self.docstring {
            write!(f, "{}\n\n", docstring)?;
        }

        for import in &self.imports {
            write!(f, "{}\n", import)?;
        }

        for import_from in &self.import_froms {
            write!(f, "{}\n", import_from)?;
        }

        for class in &self.classes {
            write!(f, "{}\n\n", class)?;
        }

        for function in &self.functions {
            write!(f, "{}\n\n", function)?;
        }

        Ok(())
    }
}

impl Component for Module {}
