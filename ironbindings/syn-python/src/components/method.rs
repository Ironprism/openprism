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

use crate::python_token::Token;

use super::{decorator::Decorator, docstring::Arg, function::Function};

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
        self.method_decorators.contains(&MethodDecorators::StaticMethod)
    }

    /// Returns whether the method is a class method.
    pub fn is_class_method(&self) -> bool {
        self.method_decorators.contains(&MethodDecorators::ClassMethod)
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
    fn from(m: Method) -> Function {
        let mut decorators = m.decorators;
        for method_decorator in m.method_decorators {
            decorators.push(method_decorator.into());
        }
        let mut docstring: Option<Docstring> = m.docstring;

        if let (Some(first_argument), Some(docstring)) = (m.first_argument(), docstring.as_mut()) {
            docstring.add_arg(first_argument);
        }

        Function::new(m.name, docstring, m.body, decorators)
    }
}