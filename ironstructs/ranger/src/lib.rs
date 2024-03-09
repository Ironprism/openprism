#![deny(unconditional_recursion)]
#![cfg_attr(not(feature = "rayon"), no_std)]

use core::ops::{Add, Sub};
use irontraits::{One, To, Zero};
use core::ops::Range;

#[derive(Debug, Clone)]
pub struct Ranger<I> {
    start: I,
    end: I,
}

impl<I: PartialOrd> From<Range<I>> for Ranger<I> {
    #[inline(always)]
    fn from(range: Range<I>) -> Self {
        Self::new(range.start, range.end)
    }
}

impl<I> From<Ranger<I>> for Range<I> {
    #[inline(always)]
    fn from(ranger: Ranger<I>) -> Self {
        ranger.start..ranger.end
    }
}

impl<I: PartialOrd> Ranger<I> {
    #[inline(always)]
    pub fn new(start: I, end: I) -> Self {
        assert!(start <= end);
        Self { start, end }
    }
}

impl<I: Copy> Ranger<I> {
    #[inline(always)]
    pub fn start(&self) -> I {
        self.start
    }

    #[inline(always)]
    pub fn end(&self) -> I {
        self.end
    }
}

impl<I: PartialOrd> Ranger<I>
where
    I: Zero,
{
    #[inline(always)]
    pub fn from_end(end: I) -> Self {
        Self::new(I::ZERO, end)
    }
}

impl<I: One + Add<Output = I> + PartialOrd + Copy> Iterator for Ranger<I> {
    type Item = I;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let start = self.start;
            self.start = self.start + I::ONE;
            Some(start)
        } else {
            None
        }
    }
}

impl<I: One + Add<Output = I> + Sub<Output = I> + To<usize> + PartialOrd + Copy> ExactSizeIterator
    for Ranger<I>
{
    #[inline(always)]
    fn len(&self) -> usize {
        (self.end - self.start).to()
    }
}

impl<I: One + Add<Output = I> + Sub<Output = I> + PartialOrd + Copy> DoubleEndedIterator
    for Ranger<I>
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end = self.end - I::ONE;
            Some(self.end)
        } else {
            None
        }
    }
}

#[cfg(feature = "rayon")]
pub mod into_par_iter;
