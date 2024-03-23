/// Module defining the class component for Python projects.
use crate::python_token::Token;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use std::fmt;

use super::{decorator::Decorator, method::Method};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Class {
    name: Token,
    parents: Vec<Class>,
    decorators: Vec<Decorator>,
    methods: Vec<Method>,
}

impl Class {
    pub fn new(name: Token) -> Class {
        Class { name, parents: Vec::new(), decorators: Vec::new(), methods: Vec::new() }
    }

    pub fn add_method(&mut self, method: Method) {
        self.methods.push(method);
    }

    pub fn add_decorator(&mut self, decorator: Decorator) {
        self.decorators.push(decorator);
    }


    pub fn add_parent(&mut self, parent: Class) {
        self.parents.push(parent);
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for decorator in &self.decorators {
            write!(f, "{}\n", decorator)?;
        }
        write!(f, "class {}", self.name)?;
        let mut parents = self.parents.iter();
        let last_parent: Option<&Class> = parents.next_back();
        if let Some(last_parent) = last_parent {
            write!(f, "(")?;
            for parent in parents {
                write!(f, "{}, ", parent.name)?;
            }
            write!(f, "{})", last_parent)?;
        }
        write!(f, ":\n\n")?;
        for method in &self.methods {
            write!(f, "{}\n", method)?;
        }
        Ok(())
    }
}