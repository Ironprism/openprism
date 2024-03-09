#[cfg_attr(feature = "std", allow(unused_imports))]
use crate::Vec;
use crate::*;

impl<T> Iter for Vec<T> {
    type Item = T;
    type Iter<'a> = core::slice::Iter<'a, T>
    where
        Self: 'a;
    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }
}

impl<T> IterMut for Vec<T> {
    type IterMut<'a> = core::slice::IterMut<'a, T>
    where
        Self: 'a;
    #[inline(always)]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.as_mut_slice().iter_mut()
    }
}

impl<T> Sequence for Vec<T> {}

impl<T> SequenceMut for Vec<T> {}

impl<T> SequenceLen for Vec<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<T> SequenceAllocable for Vec<T> {
    #[inline(always)]
    fn defaulted(len: usize) -> Self 
    where
        Self::Item: Default
    {
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(Self::Item::default());
        }
        res
    }

    #[inline(always)]
    unsafe fn uninitialized(len: usize) -> Self {
        let mut res = Vec::with_capacity(len);
        #[allow(clippy::uninit_vec)]
        res.set_len(len);
        res
    }

    #[inline(always)]
    fn empty() -> Self {
        Vec::new()
    }
}

impl<T: Clone> SequenceRandomAccess for Vec<T> {
    #[inline(always)]
    fn get(&self, index: usize) -> Self::Item {
        self.as_slice()[index].clone()
    }
}

impl<T: Clone> SequenceRandomAccessMut for Vec<T> {
    #[inline(always)]
    fn set(&mut self, index: usize, value: Self::Item) {
        self.as_mut_slice()[index] = value;
    }
}
