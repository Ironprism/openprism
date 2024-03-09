pub mod cloned;
pub mod copied;
pub mod skip;
pub mod take;

pub mod prelude {
    pub use crate::cloned::Cloned;
    pub use crate::copied::Copied;
    pub use crate::skip::Skip;
    pub use crate::take::Take;
}
