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

use super::component::Component;
use super::typing::Typing;
use anyhow::Result;

pub struct Arg {
    name: String,
    typing: Typing
}

impl Arg {
    pub fn new(name: String, typing: Typing) -> Result<Arg> {
        if name.is_empty() {
            return Err(anyhow::anyhow!("Arg name cannot be empty"));
        }
        Ok(Arg { name, typing })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn typing(&self) -> &Typing {
        &self.typing
    }
}

impl ToString for Arg {
    fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.typing.to_string())
    }
}

impl Component for Arg {}

pub struct DocArg {
    arg: Arg,
    description: String
}

impl DocArg {
    pub fn new(arg: Arg, description: String) -> Result<DocArg> {
        if description.is_empty() {
            return Err(anyhow::anyhow!("Description cannot be empty"));
        }
        Ok(DocArg { arg, description })
    }
}

impl ToString for DocArg {
    fn to_string(&self) -> String {
        format!("{} ({}) - {}", self.arg.name, self.arg.typing.to_string(), self.description)
    }
}

impl Component for DocArg {}

pub struct Docstring {
    summary: String,
    description: String,
    args: Vec<DocArg>,
    returns: Option<DocArg>,
    raises: Vec<DocArg>
}

impl Docstring {
    pub fn new(summary: String, description: String, args: Vec<DocArg>, returns: Option<DocArg>, raises: Vec<DocArg>) -> Result<Docstring> {
        if summary.is_empty() {
            return Err(anyhow::anyhow!("Summary cannot be empty"));
        }
        if description.is_empty() {
            return Err(anyhow::anyhow!("Description cannot be empty"));
        }
        Ok(Docstring { summary, description, args, returns, raises })
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn args(&self) -> &Vec<DocArg> {
        &self.args
    }

    pub fn returns(&self) -> &Option<DocArg> {
        &self.returns
    }

    pub fn raises(&self) -> &Vec<DocArg> {
        &self.raises
    }
}

impl ToString for Docstring {
    fn to_string(&self) -> String {
        let mut s = format!("\"\"\"\n{}\n\n{}\n\n", self.summary, self.description);
        if !self.args.is_empty() {
            s.push_str("Args:\n");
            for arg in &self.args {
                s.push_str(&format!("    {}\n", arg.to_string()));
            }
            s.push_str("\n");
        }
        if let Some(returns) = &self.returns {
            s.push_str("Returns:\n");
            s.push_str(&format!("    {}\n\n", returns.to_string()));
        }
        if !self.raises.is_empty() {
            s.push_str("Raises:\n");
            for arg in &self.raises {
                s.push_str(&format!("    {}\n", arg.to_string()));
            }
        }
        s.push_str("\"\"\"");
        s
    }
}

impl Component for Docstring {}