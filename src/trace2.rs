/// Iterator adapter for current state and previous state tracking.
///
/// This struct is created by the [`trace2`](crate::IteratorSimpleScanExt::trace2)
/// method on [`IteratorSimpleScanExt`](crate::IteratorSimpleScanExt). See its
/// documentation for more.
#[derive(Clone)]
pub struct Trace2<I, St, F> {
    iter: I,
    state: St,
    f: F,
}

impl<I, St, F> Trace2<I, St, F> {
    pub(crate) fn new(iter: I, state: St, f: F) -> Self {
        Self { iter, state, f }
    }
}

impl<I, St, F> Iterator for Trace2<I, St, F>
where
    I: Iterator,
    St: Clone,
    F: FnMut(&St, I::Item) -> St,
{
    type Item = (St, St);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.state.clone();
        let x = self.iter.next()?;
        let y = (self.f)(&self.state, x);
        self.state = y.clone();
        Some((prev, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
