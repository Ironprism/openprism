//! Module defining the function component for Python projects.
use super::component::Component;
use super::decorator::Decorator;
use super::docstring::DocArg;
use super::docstring::Docstring;
use crate::python_token::Token;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Function {
    name: Token,
    docstring: Option<Docstring>,
    body: String,
    decorators: Vec<Decorator>,
}

impl Function {
    pub fn new(name: Token, body: String) -> Function {
        Function {
            name,
            docstring: None,
            body,
            decorators: Vec::new(),
        }
    }

    pub fn set_docstring(&mut self, docstring: Docstring) {
        self.docstring = Some(docstring);
    }

    pub fn set_docstring_summary(&mut self, summary: String) -> Result<(), String> {
        if let Some(docstring) = &mut self.docstring {
            docstring.set_summary(summary)?;
        } else {
            self.docstring = Some(Docstring::default());
            self.set_docstring_summary(summary)?;
        }
        Ok(())
    }

    pub fn add_documented_argument(&mut self, doc_arg: DocArg) -> Result<(), String> {
        if self.docstring.is_none() {
            return Err(concat!(
                "Cannot add a documented argument to a function without a docstring. ",
                "First set the docstring summary using `set_docstring_summary`."
            )
            .to_string());
        }

        if let Some(docstring) = &mut self.docstring {
            docstring.add_arg(doc_arg);
        }
        Ok(())
    }

    pub fn add_decorator(&mut self, decorator: Decorator) -> Result<(), String> {
        if self.decorators.contains(&decorator) {
            return Err(format!(
                "The function already has the decorator `{}`.",
                decorator
            ));
        }

        self.decorators.push(decorator);

        Ok(())
    }

    pub fn add_decorators(&mut self, decorators: Vec<Decorator>) -> Result<(), String> {
        for decorator in decorators {
            self.add_decorator(decorator)?;
        }

        Ok(())
    }
}

impl Component for Function {}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for decorator in &self.decorators {
            write!(f, "{}\n", decorator)?;
        }
        if let Some(docstring) = &self.docstring {
            write!(f, "{}\n", docstring)?;
        }
        write!(f, "def {}(", self.name)?;
        if let Some(docstring) = &self.docstring {
            let mut doc_args = docstring.args().iter();
            let last_doc_arg: Option<&DocArg> = doc_args.next_back();
            if let Some(last_doc_arg) = last_doc_arg {
                for doc_arg in doc_args {
                    write!(f, "{}, ", doc_arg.arg())?;
                }
                write!(f, "{}", last_doc_arg.arg())?;
            }
        }

        write!(f, ")")?;
        if let Some(returns) = &self
            .docstring
            .as_ref()
            .and_then(|docstring: &Docstring| -> Option<DocArg> { docstring.returns().clone() })
        {
            write!(f, " -> {}", returns.arg())?;
        }
        write!(f, ":\n")?;

        write!(f, "{}", self.body)
    }
}
