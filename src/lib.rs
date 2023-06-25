/*! Provider of [IteratorSimpleScanExt].

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*
*/

#![no_std]

use core::{marker::PhantomData, mem};

/// Extension Trait of [Iterator] for simple scan operations.
///
/// Provides more simplified versions of the [scan](Iterator::scan) method.
/// They are not as flexible as `scan`, but allows for shorter code in
/// certain cases.
pub trait IteratorSimpleScanExt: Iterator {
    /// Simplified version of the [scan](Iterator::scan) method.
    /// The process is focused solely on internal state tracking.
    ///
    /// The following is what differs from `scan`.
    ///
    /// * Iteration cannot be interrupted.
    /// * Internal state needs to be [Clone].
    /// * Output items can only be the same type of the internal state.
    /// * Internal state updates can be done with argument and return value
    ///   conversions instead of mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::simple_scan::IteratorSimpleScanExt;
    /// let result = (0..10).trace(0, |s, x| s + x);
    /// let expect = (0..10).scan(0, |s, x| {
    ///     *s += x;
    ///     Some(*s)
    /// });
    ///
    /// assert!(result.eq(expect));
    /// ```
    fn trace<St, F>(self, state: St, f: F) -> Trace<Self, St, F>
    where
        Self: Sized,
        St: Clone,
        F: FnMut(&St, Self::Item) -> St,
    {
        Trace::new(self, state, f)
    }

    /// An advanced version of the [trace](Self::trace) method.
    /// In addition to the internal state, the input content is also tracked.
    ///
    /// The items in the result iterators are tuple, with the first element
    /// being the internal state, and the second element being the item from
    /// the input source iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::simple_scan::IteratorSimpleScanExt;
    /// let result = (0..10).trace2(0, |s, x| s + x);
    /// let expect = (0..10).scan(0, |s, x| {
    ///     *s += x;
    ///     Some((*s, x))
    /// });
    ///
    /// assert!(result.eq(expect));
    /// ```
    fn trace2<St, F>(self, state: St, f: F) -> Trace2<Self, St, F>
    where
        Self: Sized,
        Self::Item: Clone,
        St: Clone,
        F: FnMut(&St, Self::Item) -> St,
    {
        Trace2::new(self, state, f)
    }

    /// Returns an iterator adapter that takes the difference between
    /// the input item and the previous input item.
    ///
    /// # Arguments
    ///
    /// * `ini` -
    ///     Alternate value of the previous element used only in the first iteration.
    /// * `f` -
    ///     Difference processing. The first argument is the current input item and
    ///     the second argument is the previous input item.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::mem;
    /// # use crate::simple_scan::IteratorSimpleScanExt;
    /// let result = (0..10).diff(0, |c, p| c - p);
    /// let expect = (0..10).scan(0, |s, x| {
    ///     let p = mem::replace(s, x);
    ///     Some(x - p)
    /// });
    ///
    /// assert!(result.eq(expect));
    /// ```
    fn diff<F, D>(self, ini: Self::Item, f: F) -> Diff<Self, F, D>
    where
        Self: Sized,
        Self::Item: Clone,
        F: FnMut(Self::Item, Self::Item) -> D,
    {
        Diff::new(self, ini, f)
    }
}

impl<T: Iterator> IteratorSimpleScanExt for T {
    // NOP.
}

/// Iterator adapter for state tracking.
///
/// This struct is created by the [trace](IteratorSimpleScanExt::trace) method
/// on [IteratorSimpleScanExt]. See its documentation for more.
#[derive(Clone)]
pub struct Trace<I, St, F> {
    iter: I,
    state: St,
    f: F,
}

impl<I, St, F> Trace<I, St, F> {
    fn new(iter: I, state: St, f: F) -> Self {
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

/// Iterator adapter for state and input tracking.
///
/// This struct is created by the [trace2](IteratorSimpleScanExt::trace2) method
/// on [IteratorSimpleScanExt]. See its documentation for more.
#[derive(Clone)]
pub struct Trace2<I, St, F> {
    iter: I,
    state: St,
    f: F,
}

impl<I, St, F> Trace2<I, St, F> {
    fn new(iter: I, state: St, f: F) -> Self {
        Self { iter, state, f }
    }
}

impl<I, St, F> Iterator for Trace2<I, St, F>
where
    I: Iterator,
    I::Item: Clone,
    St: Clone,
    F: FnMut(&St, I::Item) -> St,
{
    type Item = (St, I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        let y = (self.f)(&self.state, x.clone());
        self.state = y.clone();
        Some((y, x))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// Iterator adapter for item diff tracking.
///
/// This struct is created by the [diff](IteratorSimpleScanExt::diff) method
/// on [IteratorSimpleScanExt]. See its documentation for more.
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
    fn new(iter: I, prev: I::Item, f: F) -> Self {
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
