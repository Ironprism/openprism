//! Module defining the decorator component for Python projects.
use super::component::Component;
use crate::python_token::Token;

/// Enum defining custom decorators we made up to better handle the complexity of Rust to Python bindings.
pub enum Decorator {
    /// The `Unsafe` decorator is used to mark a function as unsafe.
    /// In python there is no concept of unsafe code, so this decorator is used to mark a function as unsafe
    /// when it is marked as pub unsafe in Rust.
    Unsafe,
    Custom(Token),
}

impl From<Decorator> for Token {
    fn from(d: Decorator) -> Token {
        match d {
            Decorator::Unsafe => Token::try_from("unsafe").unwrap(),
            Decorator::Custom(token) => token,
        }
    }
}

impl From<Token> for Decorator {
    fn from(t: Token) -> Decorator {
        match t.value() {
            "unsafe" => Decorator::Unsafe,
            _ => Decorator::Custom(t),
        }
    }
}

impl Display for Decorator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Decorator::Unsafe => write!(f, "@unsafe"),
            Decorator::Custom(name) => write!(f, "@{}", name),
        }
    }
}

impl Component for Decorator {}