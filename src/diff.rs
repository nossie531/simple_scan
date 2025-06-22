//! Provider of [`Diff`].

use crate::msg;
use core::{marker::PhantomData, mem};

/// Iterator adapter for item diff tracking.
///
/// This struct is created by the [`diff`](crate::IteratorSimpleScanExt::diff)
/// method on [`IteratorSimpleScanExt`](crate::IteratorSimpleScanExt). See its
/// documentation for more.
#[must_use = msg::iter_must_use!()]
#[derive(Clone)]
pub struct Diff<I: Iterator, F, D> {
    iter: I,
    prev: I::Item,
    f: F,
    d: PhantomData<D>,
}

impl<I, F, D> Diff<I, F, D>
where
    I: Iterator,
{
    pub(crate) fn new(iter: I, prev: I::Item, f: F) -> Self {
        Self {
            iter,
            prev,
            f,
            d: PhantomData,
        }
    }
}

impl<I, F, D> Iterator for Diff<I, F, D>
where
    I: Iterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> D,
{
    type Item = D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.iter.next()?;
        let prev = mem::replace(&mut self.prev, curr.clone());
        let diff = (self.f)(curr, prev);
        Some(diff)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
