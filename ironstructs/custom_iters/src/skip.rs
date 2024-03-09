//! Module providing a version of the Skip struct that works on objects implementing IntoSSIterator.

use irontraits::{Iter, IterMut};
use rayon::prelude::*;

pub struct Skip<I> {
    iter: I,
    n: usize,
}

unsafe impl<I: Send> Send for Skip<I> {}
unsafe impl<I: Sync> Sync for Skip<I> {}

impl<I> Skip<I> {
    pub fn new(iter: I, n: usize) -> Self {
        Skip { iter, n }
    }
}

impl<I> AsRef<I> for Skip<I> {
    fn as_ref(&self) -> &I {
        &self.iter
    }
}

impl<I> AsMut<I> for Skip<I> {
    fn as_mut(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<I> IntoIterator for Skip<I>
where
    I: IntoIterator,
{
    type Item = <I as IntoIterator>::Item;
    type IntoIter = core::iter::Skip<<I as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter.into_iter().skip(self.n)
    }
}

impl<I> Iter for Skip<I>
where
    I: Iter,
{
    type Item = I::Item;
    type Iter<'a> = core::iter::Skip<I::Iter<'a>>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iter.iter().skip(self.n)
    }
}

impl<I> IterMut for Skip<I>
where
    I: IterMut,
{
    type IterMut<'a> = core::iter::Skip<<I as IterMut>::IterMut<'a>>
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter.iter_mut().skip(self.n)
    }
}

impl<T> IntoParallelIterator for Skip<T>
where
    T: IntoParallelIterator,
    <T as IntoParallelIterator>::Iter: IndexedParallelIterator,
{
    type Item = <T::Iter as ParallelIterator>::Item;
    type Iter = rayon::iter::Skip<T::Iter>;

    fn into_par_iter(self) -> Self::Iter {
        self.iter.into_par_iter().skip(self.n)
    }
}
