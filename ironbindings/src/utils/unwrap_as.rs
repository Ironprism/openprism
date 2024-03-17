use rustdoc_types::*;

pub trait UnwrapAs {
    fn is<T>(self) -> bool
    where
        Self: UnwrapAsImpl<T>;

    fn unwrap_as<T>(self) -> T
    where
        Self: UnwrapAsImpl<T>;
}

pub trait UnwrapAsImpl<T> {
    fn is_impl(self) -> bool;

    fn unwrap_as_impl(self) -> T;
}

impl UnwrapAs for rustdoc_types::Item {
    fn is<T>(self) -> bool
        where
            Self: UnwrapAsImpl<T> {
        self.is_impl()
    }

    fn unwrap_as<U>(self) -> U
    where
        Self: UnwrapAsImpl<U>
    {
        self.unwrap_as_impl()
    }
}

impl UnwrapAs for &rustdoc_types::Item {
    fn is<T>(self) -> bool
        where
            Self: UnwrapAsImpl<T> {
        self.is_impl()
    }
    fn unwrap_as<U>(self) -> U
    where
        Self: UnwrapAsImpl<U>
    {
        self.unwrap_as_impl()
    }
}

impl UnwrapAs for rustdoc_types::ItemEnum {
    fn is<T>(self) -> bool
        where
            Self: UnwrapAsImpl<T> {
        self.is_impl()
    }
    fn unwrap_as<U>(self) -> U
    where
        Self: UnwrapAsImpl<U>
    {
        self.unwrap_as_impl()
    }
}

impl UnwrapAs for &rustdoc_types::ItemEnum {
    fn is<T>(self) -> bool
        where
            Self: UnwrapAsImpl<T> {
        self.is_impl()
    }
    fn unwrap_as<U>(self) -> U
    where
        Self: UnwrapAsImpl<U>
    {
        self.unwrap_as_impl()
    }
}


impl<T> UnwrapAsImpl<T> for rustdoc_types::Item 
where
    rustdoc_types::ItemEnum: UnwrapAsImpl<T>
{   
    fn is_impl(self) -> bool {
        self.inner.is_impl()
    }

    fn unwrap_as_impl(self) -> T {
        self.inner.unwrap_as_impl()
    }
}

impl<'a, T> UnwrapAsImpl<&'a T> for &'a rustdoc_types::Item 
where
   &'a rustdoc_types::ItemEnum: UnwrapAsImpl<&'a T>
{
    fn is_impl(self) -> bool {
        (&self.inner).is_impl()
    }

    fn unwrap_as_impl(self) -> &'a T {
        (&self.inner).unwrap_as_impl()
    }
}

macro_rules! impl_unwrap_as {
    ($($var:ident => $t:ty),*) => {
        $(
            impl UnwrapAsImpl<$t> for ItemEnum {
                fn is_impl(self) -> bool {
                    matches!(self, ItemEnum::$var(_))
                }
                fn unwrap_as_impl(self) -> $t {
                    match self {
                        ItemEnum::$var(f) => f,
                        _ => panic!("Expected {} item but got {:?}", stringify!($t), self),
                    }
                }
            }
            impl<'a> UnwrapAsImpl<&'a $t> for &'a ItemEnum {
                fn is_impl(self) -> bool {
                    matches!(self, ItemEnum::$var(_))
                }
                fn unwrap_as_impl(self) -> &'a $t {
                    match self {
                        ItemEnum::$var(f) => f,
                        _ => panic!("Expected {} item but got {:?}", stringify!($t), self),
                    }
                }
            }
        )*
    };
}

impl_unwrap_as!(
    Module => rustdoc_types::Module,
    Import => rustdoc_types::Import,
    Union => rustdoc_types::Union,
    Struct => rustdoc_types::Struct,
    StructField => rustdoc_types::Type,
    Enum => rustdoc_types::Enum,
    Variant => rustdoc_types::Variant,
    Function => rustdoc_types::Function,
    Trait => rustdoc_types::Trait,
    TraitAlias => rustdoc_types::TraitAlias,
    Impl => rustdoc_types::Impl,
    TypeAlias => rustdoc_types::TypeAlias,
    OpaqueTy => rustdoc_types::OpaqueTy,
    Constant => rustdoc_types::Constant,
    Static => rustdoc_types::Static,
    Macro => String,
    ProcMacro => rustdoc_types::ProcMacro,
    Primitive => rustdoc_types::Primitive
);