//! Provider of [`Trace`].

use crate::msg;

/// Iterator adapter for state tracking.
///
/// This struct is created by the [`trace`](crate::IteratorSimpleScanExt::trace)
/// method on [`IteratorSimpleScanExt`](crate::IteratorSimpleScanExt). See its
/// documentation for more.
#[must_use = msg::iter_must_use!()]
#[derive(Clone)]
pub struct Trace<I, St, F> {
    iter: I,
    state: St,
    f: F,
}

impl<I, St, F> Trace<I, St, F> {
    pub(crate) fn new(iter: I, state: St, f: F) -> Self {
        Self { iter, state, f }
    }
}

impl<I, St, F> Iterator for Trace<I, St, F>
where
    I: Iterator,
    St: Clone,
    F: FnMut(&St, I::Item) -> St,
{
    type Item = St;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        let y = (self.f)(&self.state, x);
        self.state = y.clone();
        Some(y)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
