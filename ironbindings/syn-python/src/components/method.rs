//! Submodule providing the `Method` component.
//!
//! A method differs from a function in that it is defined within a class.
//! Differently from a function, depending from the potential decorators that
//! may be applied to it, a method may have a different signature. Without
//! decorators, the first argument of a method is always `self`, which is
//! a reference to the instance of the class that is calling the method.
//!
//! When a method is defined as a class method, the first argument is `cls`,
//! which is a reference to the class itself. When a method is defined as a
//! static method, it does not have any reference to the class or the instance.
//!
//! In order to avoid code duplication, we implement the [`From`] trait to convert
//! a [`Method`] into a [`Function`], adding the first argument as needed.

use super::{
    decorator::Decorator,
    docstring::{Arg, DocArg, Docstring},
    function::Function,
};
use crate::python_token::Token;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MethodDecorators {
    StaticMethod,
    ClassMethod,
    Final,
    Abstract,
    Overload,
    Property,
    Setter(Token),
}

impl From<MethodDecorators> for Token {
    fn from(d: MethodDecorators) -> Token {
        match d {
            MethodDecorators::StaticMethod => Token::try_from("staticmethod").unwrap(),
            MethodDecorators::ClassMethod => Token::try_from("classmethod").unwrap(),
            MethodDecorators::Final => Token::try_from("final").unwrap(),
            MethodDecorators::Abstract => Token::try_from("abstractmethod").unwrap(),
            MethodDecorators::Overload => Token::try_from("overload").unwrap(),
            MethodDecorators::Property => Token::try_from("property").unwrap(),
            MethodDecorators::Setter(token) => token,
        }
    }
}

impl From<Token> for MethodDecorators {
    fn from(t: Token) -> MethodDecorators {
        match t.value() {
            "staticmethod" => MethodDecorators::StaticMethod,
            "classmethod" => MethodDecorators::ClassMethod,
            "final" => MethodDecorators::Final,
            "abstractmethod" => MethodDecorators::Abstract,
            "overload" => MethodDecorators::Overload,
            "property" => MethodDecorators::Property,
            _ => MethodDecorators::Setter(t),
        }
    }
}

impl From<MethodDecorators> for Decorator {
    fn from(d: MethodDecorators) -> Decorator {
        Decorator::Custom(d.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Method {
    name: Token,
    docstring: Option<Docstring>,
    body: String,
    method_decorators: Vec<MethodDecorators>,
    decorators: Vec<Decorator>,
}

impl Method {
    /// Returns whether the method is a static method.
    pub fn is_static_method(&self) -> bool {
        self.method_decorators
            .contains(&MethodDecorators::StaticMethod)
    }

    /// Returns whether the method is a class method.
    pub fn is_class_method(&self) -> bool {
        self.method_decorators
            .contains(&MethodDecorators::ClassMethod)
    }

    /// Set the provided docstring
    pub fn set_docstring(&mut self, docstring: Docstring) {
        self.docstring = Some(docstring);
    }

    /// Set the provided docstring summary
    pub fn set_docstring_summary(&mut self, summary: String) -> Result<(), String> {
        if let Some(docstring) = &mut self.docstring {
            docstring.set_summary(summary)?;
        } else {
            self.docstring = Some(Docstring::default());
            self.set_docstring_summary(summary)?;
        }
        Ok(())
    }

    /// Add a documented argument to the docstring
    pub fn add_documented_argument(&mut self, doc_arg: DocArg) -> Result<(), String> {
        if self.docstring.is_none() {
            return Err("Cannot add a documented argument to a method without a docstring. First set the docstring summary using `set_docstring_summary`.".to_string());
        }

        if doc_arg.is_implicit() {
            return Err("Cannot add an implicit argument to a method.".to_string());
        }

        if let Some(docstring) = &mut self.docstring {
            docstring.add_arg(doc_arg);
        }
        Ok(())
    }

    /// Returns the first argument of the method.
    pub fn first_argument(&self) -> Option<Arg> {
        if self.is_class_method() {
            Some(Arg::Cls)
        } else if self.is_static_method() {
            None
        } else {
            Some(Arg::Slf)
        }
    }
}

impl From<Method> for Function {
    fn from(mut m: Method) -> Function {
        let first_argument = m.first_argument();
        let mut decorators = m.decorators;
        for method_decorator in m.method_decorators {
            decorators.push(method_decorator.into());
        }

        if let (Some(first_argument), Some(docstring)) = (first_argument, m.docstring.as_mut()) {
            docstring.prepend_implicit_arg(first_argument).unwrap();
        }

        let mut function = Function::new(m.name, m.body);

        if let Some(docstring) = m.docstring {
            function.set_docstring(docstring);
        }
        function.add_decorators(decorators).unwrap();

        function
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let function: Function = self.clone().into();
        write!(f, "{}", function)
    }
}
