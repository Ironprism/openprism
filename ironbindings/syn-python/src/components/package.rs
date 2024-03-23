//! Submodule providing the struct `Package`, which represents a Python package.
use super::module::Module;
use crate::python_token::Token;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

pub trait MagicVariable: Display {
    fn name(&self) -> &'static str;

    fn fmt_as_assignment(&self) -> String {
        format!("{} = {}", self.name(), self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl MagicVariable for Version {
    fn name(&self) -> &'static str {
        "__version__"
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    name: String,
}

impl MagicVariable for Author {
    fn name(&self) -> &'static str {
        "__author__"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[repr(transparent)]
pub struct Authors {
    authors: Vec<Author>,
}

impl MagicVariable for Authors {
    fn name(&self) -> &'static str {
        if self.authors.len() == 1 {
            "__author__"
        } else {
            "__authors__"
        }
    }
}

impl Display for Authors {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.authors.len() == 1 {
            return write!(f, "{}", self.authors[0]);
        }
        write!(
            f,
            "[{}]",
            self.authors
                .iter()
                .map(|author| author.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum License {
    MIT,
    Apache2,
    GPL3,
}

impl Display for License {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            License::MIT => write!(f, "MIT"),
            License::Apache2 => write!(f, "Apache 2.0"),
            License::GPL3 => write!(f, "GPLv3"),
        }
    }
}

impl MagicVariable for License {
    fn name(&self) -> &'static str {
        "__license__"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyRight {
    year: u32,
    company: String,
}

impl Display for CopyRight {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Copyright {}, {}", self.year, self.company)
    }
}

impl MagicVariable for CopyRight {
    fn name(&self) -> &'static str {
        "__copyright__"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Maintainer {
    author: Author,
}

impl MagicVariable for Maintainer {
    fn name(&self) -> &'static str {
        "__maintainer__"
    }
}

impl Display for Maintainer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.author)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaintainerEmail {
    email: String,
}

impl MaintainerEmail {
    pub fn new(email: &str) -> Result<MaintainerEmail, String> {
        if !EmailAddress::is_valid(email) {
            return Err(format!("{} is not a valid email address", email));
        }

        Ok(MaintainerEmail {
            email: email.to_string(),
        })
    }
}

impl MagicVariable for MaintainerEmail {
    fn name(&self) -> &'static str {
        "__email__"
    }
}

impl Display for MaintainerEmail {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Planning,
    PreAlpha,
    Alpha,
    Beta,
    ProductionStable,
    Mature,
    Inactive,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Status::Planning => write!(f, "Planning"),
            Status::PreAlpha => write!(f, "Pre-Alpha"),
            Status::Alpha => write!(f, "Alpha"),
            Status::Beta => write!(f, "Beta"),
            Status::ProductionStable => write!(f, "Production/Stable"),
            Status::Mature => write!(f, "Mature"),
            Status::Inactive => write!(f, "Inactive"),
        }
    }
}

impl MagicVariable for Status {
    fn name(&self) -> &'static str {
        "__status__"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Package {
    name: Token,
    modules: Vec<Module>,
    version: Version,
    authors: Authors,
    credits: Option<Authors>,
    license: License,
    copy_right: Option<CopyRight>,
    maintainer: Option<Maintainer>,
    maintainer_email: Option<MaintainerEmail>,
    status: Option<Status>
}
