use crate::Ranger;

use core::ops::{Add, Sub};
use irontraits::{One, To};
use rayon::iter::plumbing::{bridge, Producer};
use rayon::prelude::*;

impl<I: One + Add<Output = I> + Sub<Output = I> + To<usize> + PartialOrd + Send + Sync + Copy>
    Producer for Ranger<I>
where
    usize: To<I>,
{
    type Item = I;
    type IntoIter = Self;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self
    }

    #[inline(always)]
    fn split_at(self, index: usize) -> (Self, Self) {
        let middle = self.start + index.to();
        let left = Self::new(self.start, middle);
        let right = Self::new(middle, self.end);
        (left, right)
    }
}

impl<I: One + Add<Output = I> + Sub<Output = I> + To<usize> + PartialOrd + Send + Sync + Copy>
    ParallelIterator for Ranger<I>
where
    usize: To<I>,
{
    type Item = I;

    #[inline(always)]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    #[inline(always)]
    fn opt_len(&self) -> Option<usize> {
        Some(<Self as ExactSizeIterator>::len(self))
    }
}

impl<I: One + Add<Output = I> + Sub<Output = I> + To<usize> + PartialOrd + Send + Sync + Copy>
    IndexedParallelIterator for Ranger<I>
where
    usize: To<I>,
{
    #[inline(always)]
    fn len(&self) -> usize {
        <Self as ExactSizeIterator>::len(self)
    }

    #[inline(always)]
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    #[inline(always)]
    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: rayon::iter::plumbing::ProducerCallback<Self::Item>,
    {
        callback.callback(self)
    }
}
