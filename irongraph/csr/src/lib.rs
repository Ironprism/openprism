//! Module for compressed sparse row (CSR) data structure.

#[cfg(all(not(feature = "std"), feature = "rayon"))]
compile_error!("Rayon requires std");

#[cfg_attr(not(feature = "std"), no_std)]
pub mod builders;
pub mod csr;
pub mod iter;

#[cfg(feature = "rayon")]
pub mod par_iter;

pub mod prelude {
    pub use super::builders::*;
    pub use super::csr::*;
}
