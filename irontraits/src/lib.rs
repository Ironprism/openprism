#![deny(unconditional_recursion)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::Vec;
#[cfg(feature = "std")]
pub use std::vec::Vec;

mod positive_integer;
pub use positive_integer::*;

mod iter;
pub use iter::*;

mod sequence;
pub use sequence::*;

#[cfg(any(feature = "std", feature = "alloc"))]
mod sequence_alloc;

mod to;
pub use to::*;

#[cfg(feature = "rayon")]
mod rayon_traits;
#[cfg(feature = "rayon")]
pub use rayon_traits::IntoIndexedParallelIterator;