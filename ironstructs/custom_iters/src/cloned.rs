//! Module providing a version of the Cloned struct that works on objects implementing IntoSSIterator.

use rayon::prelude::*;

#[derive(Clone, Copy)]
pub struct Cloned<I> {
    iter: I,
}

unsafe impl<I: Send> Send for Cloned<I> {}
unsafe impl<I: Sync> Sync for Cloned<I> {}

impl<I> Cloned<I> {
    pub fn new(iter: I) -> Self {
        Cloned { iter }
    }
}

impl<I> AsRef<I> for Cloned<I> {
    fn as_ref(&self) -> &I {
        &self.iter
    }
}

impl<I> AsMut<I> for Cloned<I> {
    fn as_mut(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<'a, I, T: 'a + Clone> IntoIterator for Cloned<I>
where
    I: IntoIterator<Item = &'a T>,
{
    type Item = T;
    type IntoIter = core::iter::Cloned<<I as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter.into_iter().cloned()
    }
}

impl<'a, I, T: 'a + Clone + Send + Sync> IntoParallelIterator for Cloned<I>
where
    I: IntoParallelIterator<Item = &'a T>,
{
    type Item = T;
    type Iter = rayon::iter::Cloned<<I as IntoParallelIterator>::Iter>;

    fn into_par_iter(self) -> Self::Iter {
        self.iter.into_par_iter().cloned()
    }
}
