//! Module defining the function component for Python projects.
use super::component::Component;
use super::typing::Typing;
use super::docstring::Docstring;
use super::decorator::Decorator;
use crate::python_token::Token;

pub struct Function {
    name: Token,
    docstring: Option<Docstring>,
    body: String,
    decorators: Vec<Decorator>,
}

impl Function {
    pub fn new(name: Token, docstring: Option<Docstring>, body: String, decorators: Vec<Decorator>) -> Function {
        Function { name, docstring, body, decorators }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for decorator in &self.decorators {
            write!(f, "{}\n", decorator)?;
        }
        if let Some(docstring) = &self.docstring {
            write!(f, "{}\n", docstring)?;
        }
        write!(f, "def {}(", self.name)?;
        let mut doc_args = self.docstring.args().iter();
        let last_doc_arg: Option<DocArg> = doc_args.next_back();
        if let Some(last_doc_arg) = last_doc_arg {
            for doc_arg in &doc_args {
                write!(f, "{}, ", doc_arg.arg())?;
            }
            write!(f, "{}", last_doc_arg.arg())?;
        }
        write!(f, ")");
        if let Some(returns) = &self.docstring.returns() {
            write!(f, " -> {}", returns.arg())?;
        }
        write!(f, ":\n")?;

        write!(f, "{}", self.body)
    }
}