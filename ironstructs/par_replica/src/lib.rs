//! Module providing the ParReplica struct, which given a clonable object,
//! creates an array with K replicas of the object, where K is the number of
//! threads. Then, provides operations over the replicas, such as getting the
//! replica curresponding to the current thread, or getting a reference to all
//! the replicas. It also implements Default, so that it can be used as a
//! default value for a struct.

#![cfg_attr(not(feature="rayon"), no_std)]
use core::cell::UnsafeCell;

#[cfg(not(feature="rayon"))]
extern crate alloc;
#[cfg(not(feature="rayon"))]
use alloc::vec::Vec;
#[cfg(not(feature="rayon"))]
use alloc::vec;

pub struct ParReplica<T> {
    replicas: SyncUnsafeCell<Vec<T>>,
}

struct SyncUnsafeCell<T>(UnsafeCell<T>);

unsafe impl<T> Send for SyncUnsafeCell<T> {}
unsafe impl<T> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    #[inline(always)]
    fn new(value: T) -> Self {
        SyncUnsafeCell(UnsafeCell::new(value))
    }

    #[inline(always)]
    fn get(&self) -> &T {
        unsafe { &*self.0.get() }
    }

    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    unsafe fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}

impl<T> ParReplica<T>
where
    T: Clone + Send + Sync,
{
    /// Creates a new ParReplica from the given object.
    #[inline(always)]
    pub fn new_with_thread_number(object: T, number_of_threads: usize) -> Self {
        ParReplica {
            replicas: SyncUnsafeCell::new(vec![object; number_of_threads.max(1)]),
        }
    }

    /// Returns a reference to the replica corresponding to the current thread.
    #[inline(always)]
    pub fn get_by_id(&self, thread_id: usize) -> &T {
        &self.replicas.get()[thread_id]
    }

    /// Returns a mutable reference to the replica corresponding to the current
    /// thread.
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_mut_by_id(&self, thread_id: usize) -> &mut T {
        &mut self.replicas.get_mut()[thread_id]
    }

    /// Creates a new ParReplica from the given object.
    #[cfg(feature = "rayon")]
    #[inline(always)]
    pub fn new(object: T) -> Self {
        ParReplica {
            replicas: SyncUnsafeCell::new(vec![object; rayon::current_num_threads().max(1)]),
        }
    }

    #[inline(always)]
    #[cfg(feature = "rayon")]
    fn current_thread_index(&self) -> usize {
        rayon::current_thread_index().unwrap_or(0)
    }

    /// Returns a reference to the replica corresponding to the current thread.
    #[inline(always)]
    #[cfg(feature = "rayon")]
    pub fn get(&self) -> &T {
       self.get_by_id(self.current_thread_index())
    }

    /// Returns a mutable reference to the replica corresponding to the current
    /// thread.
    #[inline(always)]
    #[cfg(feature = "rayon")]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_mut(&self) -> &mut T {
        self.get_mut_by_id(self.current_thread_index())
    }
}
