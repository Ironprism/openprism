//! Module defining the import statement for Python projects.
use crate::python_token::Token;

use super::component::Component;

pub struct Import {
    name: Token,
    alias: Option<Token>,
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.alias {
            Some(alias) => write!(f, "import {} as {}", self.name, alias),
            None => write!(f, "import {}", self.name),
        }
    }
}

pub struct ImportFrom {
    module: Token,
    names: Vec<(Token, Option<Token>)>,
}

impl Display for ImportFrom {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "from {} import ", self.module)?;
        for (name, alias) in &self.names {
            match alias {
                Some(alias) => write!(f, "{} as {}, ", name, alias),
                None => write!(f, "{}, ", name),
            }
        }
        Ok(())
    }
}

impl Component for Import {}
impl Component for ImportFrom {}
