//! Module providing a version of the Take struct that works on objects implementing IntoSSIterator.

use irontraits::{Iter, IterMut};
use rayon::prelude::*;

pub struct Take<I> {
    iter: I,
    n: usize,
}

unsafe impl<I: Send> Send for Take<I> {}
unsafe impl<I: Sync> Sync for Take<I> {}

impl<I> Take<I> {
    pub fn new(iter: I, n: usize) -> Self {
        Take { iter, n }
    }
}

impl<I> AsRef<I> for Take<I> {
    fn as_ref(&self) -> &I {
        &self.iter
    }
}

impl<I> AsMut<I> for Take<I> {
    fn as_mut(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<I> IntoIterator for Take<I>
where
    I: IntoIterator,
{
    type Item = <I as IntoIterator>::Item;
    type IntoIter = core::iter::Take<<I as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter.into_iter().take(self.n)
    }
}

impl<I: Iter> Iter for Take<I>
where
    I: Iter,
{
    type Item = I::Item;
    type Iter<'a> = core::iter::Take<I::Iter<'a>>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iter.iter().take(self.n)
    }
}

impl<I> IterMut for Take<I>
where
    I: IterMut,
{
    type IterMut<'a> = core::iter::Take<<I as IterMut>::IterMut<'a>>
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter.iter_mut().take(self.n)
    }
}

impl<T> IntoParallelIterator for Take<T>
where
    T: IntoParallelIterator,
    <T as IntoParallelIterator>::Iter: IndexedParallelIterator,
{
    type Item = <T::Iter as ParallelIterator>::Item;
    type Iter = rayon::iter::Take<T::Iter>;

    fn into_par_iter(self) -> Self::Iter {
        self.iter.into_par_iter().take(self.n)
    }
}
