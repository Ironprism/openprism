//! Module providing a version of `IntoParallelIterator` that returns an
//! `IndexedParallelIterator`.

use rayon::prelude::*;

/// Trait providing a version of `IntoParallelIterator` that returns an
/// `IndexedParallelIterator`.
pub trait IntoIndexedParallelIterator:
    IntoParallelIterator<
    Item = <Self as IntoIndexedParallelIterator>::Item,
    Iter = <Self as IntoIndexedParallelIterator>::IndexedIter,
>
{
    type Item;
    type IndexedIter: IndexedParallelIterator<Item = <Self as IntoIndexedParallelIterator>::Item>;
}

impl<T> IntoIndexedParallelIterator for T
where
    T: IntoParallelIterator,
    T::Iter: IndexedParallelIterator,
{
    type Item = <T::Iter as ParallelIterator>::Item;
    type IndexedIter = T::Iter;
}
