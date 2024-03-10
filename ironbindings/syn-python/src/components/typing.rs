//! Module defining the general typing component for Python projects.
//!
//! # Implementation details
//! A type in python may be a simple type, such as `int` or `str`, or a more complex type,
//! such as a list or a dictionary. In the case of a List or a Dict, due to the potential use
//! of type hitings from the typing module, it may contain one or more other types, which are
//! defined as part of the enum `Typing`.
use super::component::Component;

pub enum Typing {
    /// Integer type.
    Int,
    /// Float type.
    Float,
    /// String type.
    Str,
    /// Boolean type.
    Bool,
    /// None type.
    None,
    /// Any type.
    Any,
    /// Optional type, such as `Optional[int]`.
    Optional(Box<Typing>),
    /// A custom type, such as `MyClass`.
    Custom(String),
    /// A list type, such as `List[int]`.
    List(Box<Typing>),
    /// A dictionary type, such as `Dict[str, int]`.
    Dict(Box<Typing>, Box<Typing>),
    /// A tuple type, such as `Tuple[int, str]`.
    Tuple(Vec<Typing>),
    /// A union type, such as `Union[int, str]`.
    Union(Vec<Typing>),
}

impl ToString for Typing {
    fn to_string(&self) -> String {
        match self {
            Typing::Int => "int".to_string(),
            Typing::Float => "float".to_string(),
            Typing::Str => "str".to_string(),
            Typing::Bool => "bool".to_string(),
            Typing::None => "None".to_string(),
            Typing::Any => "Any".to_string(),
            Typing::Optional(t) => format!("Optional[{}]", t.to_string()),
            Typing::Custom(t) => t.to_string(),
            Typing::List(t) => format!("List[{}]", t.to_string()),
            Typing::Dict(k, v) => format!("Dict[{}, {}]", k.to_string(), v.to_string()),
            Typing::Tuple(t) => format!(
                "Tuple[{}]",
                t.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Typing::Union(t) => format!(
                "Union[{}]",
                t.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl Component for Typing {}