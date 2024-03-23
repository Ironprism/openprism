//! Module defining the docstring component for Python projects.
//!
//! Implementation details
//! ----------------------
//! The docstring format we use is the Google-style docstring format.
//! An example of a Google-style docstring is:
//!
//! ```python
//! def foo(bar: int) -> int:
//!     """Summary line.
//!
//!     Extended description of function.
//!
//!     Args:
//!         bar (int): Description of bar
//!
//!     Returns:
//!         int: Description of return value
//!     """
//!     return bar
//! ```
//!
//! The docstring is a multi-line string that is placed immediately after the function or class
//! definition. It is a string that is enclosed in triple quotes. The first line of the docstring is
//! a brief summary of the function or class. The following lines are an extended description of the
//! function or class. The next section is the Args section, which lists the arguments of the
//! function or class, along with their types and descriptions. The next section is the Returns
//! section, which lists the return value of the function or class, along with its type and
//! description.
//!
//! There are some other additional sections that we may include in the docstring, such as the
//! Raises section, which lists the exceptions that the function or class may raise. An example
//! of a docstring with a Raises section is:
//!
//! ```python
//! def foo(bar: int) -> int:
//!     """Summary line.
//!
//!     Extended description of function.
//!
//!     Args:
//!         bar (int): Description of bar
//!
//!     Returns:
//!         int: Description of return value
//!
//!     Raises:
//!         ValueError: If bar is not an integer
//!     """
//!     if not isinstance(bar, int):
//!         raise ValueError("bar must be an integer")
//!     return bar
//! ```
//!
//! We use these docstrings to document bindigs from Rust to Python. As such, there
//! may be some functions in Rust that are unsafe, i.e. they may cause undefined behavior
//! if called incorrectly. In Python, there is no concept of unsafe code, so we use a custom
//! decorator to mark these functions as unsafe. The custom decorator we use is the [`CustomDecorators::Unsafe`].
//! In methods and functions that have the `unsafe` decorator, we include a warning in the docstring
//! that the function is unsafe and may cause undefined behavior if called incorrectly when there is
//! no better description provided. Alternatively, when the description is provided, we include the
//! warning in the description.
//!
//! The following is an example of documentation of an unsafe function without a provided description:
//!
//! ```python
//! def foo(bar: int) -> int:
//!    """Summary line.
//!
//!    Extended description of function.
//!
//!    Args:
//!      bar (int): Description of bar
//!
//!    Returns:
//!      int: Description of return value
//!
//!    Raises:
//!      ValueError: If bar is not an integer
//!
//!    Safety:
//!      This function is marked as unsafe and may cause undefined behavior if called incorrectly.
//!   """
//!   if not isinstance(bar, int):
//!       raise ValueError("bar must be an integer")
//!   return bar
//! ```
//!
//! The following is an example of documentation of an unsafe function with a provided description:
//!
//! ```python
//! def foo(bar: int) -> int:
//!    """Summary line.
//!
//!    Extended description of function.
//!
//!    Args:
//!      bar (int): Description of bar
//!
//!    Returns:
//!      int: Description of return value
//!
//!    Raises:
//!      ValueError: If bar is not an integer
//!
//!    Safety:
//!      Note that we assume that bar is a positive non-zero integer and that the function may
//!      cause undefined behavior if called with a negative or zero integer.
//!
//!   """
//!   if not isinstance(bar, int):
//!       raise ValueError("bar must be an integer")
//!   
//!   return int(math.sqrt(bar))
//! ```
//!
//! [`CustomDecorators::Unsafe`]: enum.CustomDecorators.html#variant.Unsafe

use crate::python_token::Token;

use super::component::Component;
use super::typing::Typing;
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Arg {
    Slf, // self, but with a different name to avoid conflict with the Rust keyword
    Cls,
    Arg(Token, Typing),
}

impl Arg {
    pub fn new(name: Token, typing: Typing) -> Result<Arg, String> {
        if name.value() == "self" {
            return Err("Cannot use `self` as an argument name".to_string());
        }
        if name.value() == "cls" {
            return Err("Cannot use `cls` as an argument name".to_string());
        }
        Ok(Arg::Arg(name, typing))
    }

    pub fn name(&self) -> Token {
        match self {
            Arg::Slf => Token::try_from("self").unwrap(),
            Arg::Cls => Token::try_from("cls").unwrap(),
            Arg::Arg(name, _) => name.clone(),
        }
    }

    pub fn typing(&self) -> Option<&Typing> {
        match self {
            Arg::Arg(_, typing) => Some(typing),
            _ => None,
        }
    }

    pub fn is_implicit(&self) -> bool {
        match self {
            Arg::Slf | Arg::Cls => true,
            _ => false,
        }
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_implicit() {
            write!(f, "{}", self.name())
        } else {
            write!(f, "{}: {}", self.name(), self.typing().unwrap())
        }
    }
}

impl Component for Arg {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocArg {
    arg: Arg,
    description: Option<String>,
}

impl DocArg {
    pub fn new(arg: Arg, description: String) -> Result<DocArg, String> {
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        Ok(DocArg {
            arg,
            description: Some(description),
        })
    }

    pub fn new_implicit_arg(arg: Arg) -> Result<DocArg, String> {
        if !arg.is_implicit() {
            return Err(
                "Implicit arguments must be either `self` or `cls`".to_string(),
            );
        }
        Ok(DocArg {
            arg,
            description: None,
        })
    }

    pub fn is_implicit(&self) -> bool {
        self.arg.is_implicit()
    }

    pub fn arg(&self) -> &Arg {
        &self.arg
    }
}

impl Display for DocArg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.arg.is_implicit() {
            return write!(f, "");
        }
        write!(
            f,
            "{} ({}) - {}",
            self.arg.name(),
            self.arg.typing().unwrap(),
            self.description.as_ref().unwrap()
        )
    }
}

impl Component for DocArg {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Docstring {
    summary: Option<String>,
    description: Option<String>,
    args: Vec<DocArg>,
    returns: Option<DocArg>,
    raises: Vec<DocArg>,
    safety: Option<String>,
}

const DEFAULT_SAFETY_MESSAGE: &str =
    "This function is marked as unsafe and may cause undefined behavior if called incorrectly.";

impl Docstring {
    pub fn set_description(&mut self, description: String) -> Result<(), String> {
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        self.description = Some(description);
        Ok(())
    }

    pub fn set_summary(&mut self, summary: String) -> Result<(), String> {
        if summary.is_empty() {
            return Err("Summary cannot be empty".to_string());
        }
        self.summary = Some(summary);
        Ok(())
    }

    pub fn add_arg(&mut self, arg: DocArg) {
        self.args.push(arg);
    }

    pub fn set_returns(&mut self, returns: DocArg) {
        self.returns = Some(returns);
    }

    pub fn add_raise(&mut self, raise: DocArg) {
        self.raises.push(raise);
    }

    pub fn set_default_safety_message(&mut self) {
        self.set_safety_message(DEFAULT_SAFETY_MESSAGE.to_string()).unwrap();
    }

    pub fn set_safety_message(&mut self, safety: String) -> Result<(), String> {
        if safety.is_empty() {
            return Err("Safety message cannot be empty".to_string());
        }
        self.safety = Some(safety);
        Ok(())
    }

    pub fn args(&self) -> &[DocArg] {
        &self.args
    }

    pub fn prepend_implicit_arg(&mut self, arg: Arg) -> Result<(), String> {
        // We check that there are no other implicit arguments already in the list
        if self.args.iter().any(|doc_arg| doc_arg.arg().is_implicit()) {
            return Err("Cannot add an implicit argument when there is already an implicit argument in the list".to_string());
        }

        self.args.insert(0, DocArg::new_implicit_arg(arg)?);
        Ok(())
    }

    pub fn returns(&self) -> &Option<DocArg> {
        &self.returns
    }

    pub fn raises(&self) -> &[DocArg] {
        &self.raises
    }

    pub fn has_safety_message(&self) -> bool {
        self.safety.is_some()
    }
}

impl Display for Docstring {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\"\"\"\n")?;
        if let Some(summary) = &self.summary {
            write!(f, "{}\n\n", summary)?;
        }
        if let Some(description) = &self.description {
            write!(f, "{}\n\n", description)?;
        }
        if !self.args.is_empty() {
            write!(f, "Args:\n")?;
            for arg in &self.args {
                write!(f, "    {}\n", arg)?;
            }
            write!(f, "\n")?;
        }
        if let Some(returns) = &self.returns {
            write!(f, "Returns:\n")?;
            write!(f, "    {}\n\n", returns)?;
        }
        if !self.raises.is_empty() {
            write!(f, "Raises:\n")?;
            for arg in &self.raises {
                write!(f, "    {}\n", arg)?;
            }
        }
        if let Some(safety) = &self.safety {
            write!(f, "\nSafety:\n")?;
            write!(f, "    {}\n", safety)?;
        }
        write!(f, "\"\"\"")
    }
}

impl Component for Docstring {}
