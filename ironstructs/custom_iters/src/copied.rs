//! Module providing a version of the Copied struct that works on objects implementing IntoSSIterator.

use rayon::prelude::*;

pub struct Copied<I> {
    iter: I,
}

unsafe impl<I: Send> Send for Copied<I> {}
unsafe impl<I: Sync> Sync for Copied<I> {}

impl<I> Copied<I> {
    pub fn new(iter: I) -> Self {
        Copied { iter }
    }
}

impl<I> AsRef<I> for Copied<I> {
    fn as_ref(&self) -> &I {
        &self.iter
    }
}

impl<I> AsMut<I> for Copied<I> {
    fn as_mut(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<'a, I, T: 'a + Copy> IntoIterator for Copied<I>
where
    I: IntoIterator<Item = &'a T>,
{
    type Item = T;
    type IntoIter = core::iter::Copied<<I as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter.into_iter().copied()
    }
}

impl<'a, I, T: 'a + Copy + Send + Sync> IntoParallelIterator for Copied<I>
where
    I: IntoParallelIterator<Item = &'a T> + Send,
{
    type Item = T;
    type Iter = rayon::iter::Copied<<I as IntoParallelIterator>::Iter>;

    fn into_par_iter(self) -> Self::Iter {
        self.iter.into_par_iter().copied()
    }
}
