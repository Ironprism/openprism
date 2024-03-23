//! Module defining the general typing component for Python projects.
//!
//! # Implementation details
//! A type in python may be a simple type, such as `int` or `str`, or a more complex type,
//! such as a list or a dictionary. In the case of a List or a Dict, due to the potential use
//! of type hitings from the typing module, it may contain one or more other types, which are
//! defined as part of the enum `Typing`.
use super::component::Component;
use crate::python_token::Token;

pub enum NumpyType {
    /// 8-bit integer.
    Int8,
    /// 16-bit integer.
    Int16,
    /// 32-bit integer.
    Int32,
    /// 64-bit integer.
    Int64,
    /// 8-bit unsigned integer.
    UInt8,
    /// 16-bit unsigned integer.
    UInt16,
    /// 32-bit unsigned integer.
    UInt32,
    /// 64-bit unsigned integer.
    UInt64,
    /// 16-bit floating point.
    Float16,
    /// 32-bit floating point.
    Float32,
    /// 64-bit floating point.
    Float64,
    /// 128-bit floating point.
    Float128,
    /// Complex number with 64-bit floating point.
    Complex64,
    /// Complex number with 128-bit floating point.
    Complex128,
    /// Complex number with 256-bit floating point.
    Complex256,
    /// Boolean type.
    Bool,
}

impl From<NumpyType> for Token {
    fn from(t: NumpyType) -> Token {
        Token::try_from(t).unwrap()
    }
}

impl Display for NumpyType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            NumpyType::Int8 => write!(f, "np.int8"),
            NumpyType::Int16 => write!(f, "np.int16"),
            NumpyType::Int32 => write!(f, "np.int32"),
            NumpyType::Int64 => write!(f, "np.int64"),
            NumpyType::UInt8 => write!(f, "np.uint8"),
            NumpyType::UInt16 => write!(f, "np.uint16"),
            NumpyType::UInt32 => write!(f, "np.uint32"),
            NumpyType::UInt64 => write!(f, "np.uint64"),
            NumpyType::Float16 => write!(f, "np.float16"),
            NumpyType::Float32 => write!(f, "np.float32"),
            NumpyType::Float64 => write!(f, "np.float64"),
            NumpyType::Float128 => write!(f, "np.float128"),
            NumpyType::Complex64 => write!(f, "np.complex64"),
            NumpyType::Complex128 => write!(f, "np.complex128"),
            NumpyType::Complex256 => write!(f, "np.complex256"),
            NumpyType::Bool => write!(f, "bool"),
        }
    }
}

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
    /// Numpy array type.
    NumpyArray(NumpyType),
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

impl Display for Typing {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Typing::Int => write!(f, "int"),
            Typing::Float => write!(f, "float"),
            Typing::Str => write!(f, "str"),
            Typing::Bool => write!(f, "bool"),
            Typing::None => write!(f, "None"),
            Typing::Any => write!(f, "Any"),
            Typing::NumpyArray(t) => write!(f, "npt.NDArray[{}]", t),
            Typing::Optional(t) => write!(f, "Optional[{}]", t),
            Typing::Custom(t) => write!(f, "{}", t),
            Typing::List(t) => write!(f, "List[{}]", t),
            Typing::Dict(k, v) => write!(f, "Dict[{}, {}]", k, v),
            Typing::Tuple(t) => write!(f, "Tuple[{}]", t.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ")),
            Typing::Union(t) => write!(f, "Union[{}]", t.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ")),
        }
    }
}

impl Component for Typing {}