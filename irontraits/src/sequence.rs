use crate::{Iter, IterMut};
use core::ops::Range;

pub trait Sequence: Iter {}

pub trait SequenceLen: Sequence {
    fn len(&self) -> usize;

    #[inline(always)]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait SequenceMut: Sequence + IterMut {}

pub trait SequenceAllocable: SequenceLen {
    unsafe fn uninitialized(len: usize) -> Self;
}

pub trait SequenceRandomAccess: Sequence {
    /// panics if index out of bound
    fn get(&self, index: usize) -> Self::Item;

    #[inline(always)]
    #[must_use]
    fn partition_point<P>(&self, mut pred: P, range: Range<usize>) -> usize
    where
        P: FnMut(Self::Item) -> bool,
        Self::Item: Ord,
    {
        use core::cmp::Ordering::*;
        self.binary_search_by(|x| if pred(x) { Less } else { Greater }, range)
            .unwrap_or_else(|i| i)
    }

    #[inline(always)]
    fn binary_search(&self, x: &Self::Item, range: Range<usize>) -> Result<usize, usize>
    where
        Self::Item: Ord,
    {
        self.binary_search_by(|p| p.cmp(x), range)
    }

    #[inline(always)]
    fn binary_search_by_key<B, F>(
        &self,
        b: &B,
        mut f: F,
        range: Range<usize>,
    ) -> Result<usize, usize>
    where
        F: FnMut(Self::Item) -> B,
        B: Ord,
    {
        self.binary_search_by(|k| f(k).cmp(b), range)
    }

    #[inline(always)]
    fn binary_search_by<F>(&self, mut f: F, range: Range<usize>) -> Result<usize, usize>
    where
        F: FnMut(Self::Item) -> core::cmp::Ordering,
    {
        use core::cmp::Ordering::*;
        // INVARIANTS:
        // - 0 <= left <= left + size = right <= self.len()
        // - f returns Less for everything in self[..left]
        // - f returns Greater for everything in self[right..]
        let mut size = range.len();
        let mut left = range.start;
        let mut right = range.end;
        while left < right {
            let mid = left + size / 2;

            // SAFETY: the while condition means `size` is strictly positive, so
            // `size/2 < size`. Thus `left + size/2 < left + size`, which
            // coupled with the `left + size <= self.len()` invariant means
            // we have `left + size/2 < self.len()`, and this is in-bounds.
            let cmp = f(self.get(mid));

            // This control flow produces conditional moves, which results in
            // fewer branches and instructions than if/else or matching on
            // cmp::Ordering.
            // This is x86 asm for u8: https://rust.godbolt.org/z/698eYffTx.
            left = if cmp == Less { mid + 1 } else { left };
            right = if cmp == Greater { mid } else { right };
            if cmp == Equal {
                // SAFETY: same as the `get_unchecked` above
                // unsafe { core::intrinsics::assume(mid < range.end) };
                return Ok(mid);
            }

            size = right - left;
        }

        // SAFETY: directly true from the overall invariant.
        // Note that this is `<=`, unlike the assume in the `Ok` path.
        // unsafe { core::intrinsics::assume(left <= range.end) };
        Err(left)
    }
}

pub trait SequenceRandomAccessMut: Sequence {
    /// panics if index out of bound
    fn set(&mut self, index: usize, value: Self::Item);
}

impl<T> Iter for &[T] {
    type Item = T;
    type Iter<'a> = core::slice::Iter<'a, T>
    where
        Self: 'a;
    #[inline(always)]
    #[must_use]
    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self)
    }
}

impl<T> Sequence for &[T] {}

impl<T> SequenceLen for &[T] {
    #[inline(always)]
    #[must_use]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T: Clone> SequenceRandomAccess for &[T] {
    #[inline(always)]
    #[must_use]
    fn get(&self, index: usize) -> Self::Item {
        self[index].clone()
    }
}

impl<T> Iter for &mut [T] {
    type Item = T;
    type Iter<'a> = core::slice::Iter<'a, T>
    where
        Self: 'a;
    #[inline(always)]
    #[must_use]
    fn iter(&self) -> Self::Iter<'_> {
        <[T]>::iter(self)
    }
}

impl<T> IterMut for &mut [T] {
    type IterMut<'a> = core::slice::IterMut<'a, T>
    where
        Self: 'a;
    #[inline(always)]
    #[must_use]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <[T]>::iter_mut(self)
    }
}

impl<T> Sequence for &mut [T] {}

impl<T> SequenceMut for &mut [T] {}

impl<T> SequenceLen for &mut [T] {
    #[inline(always)]
    #[must_use]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T: Clone> SequenceRandomAccess for &mut [T] {
    #[inline(always)]
    #[must_use]
    fn get(&self, index: usize) -> Self::Item {
        self[index].clone()
    }
}

impl<T: Clone> SequenceRandomAccessMut for &mut [T] {
    #[inline(always)]
    fn set(&mut self, index: usize, value: Self::Item) {
        self[index] = value;
    }
}
