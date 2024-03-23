//! Module defining the import statement for Python projects.
use super::component::Component;
use crate::python_token::Token;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Import {
    import_path: Vec<Token>,
    alias: Option<Token>,
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.alias {
            Some(alias) => write!(
                f,
                "import {} as {}",
                self.import_path
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<_>>()
                    .join("."),
                alias
            ),
            None => write!(
                f,
                "import {}",
                self.import_path
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<_>>()
                    .join(".")
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportFrom {
    import_path: Vec<Token>,
    names: Vec<(Token, Option<Token>)>,
}

impl Display for ImportFrom {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "from {} import ",
            self.import_path
                .iter()
                .map(|token| token.to_string())
                .collect::<Vec<_>>()
                .join(".")
        )?;
        for (name, alias) in &self.names {
            match alias {
                Some(alias) => write!(f, "{} as {}, ", name, alias)?,
                None => write!(f, "{}, ", name)?,
            }
        }
        Ok(())
    }
}

impl Component for Import {}
impl Component for ImportFrom {}
