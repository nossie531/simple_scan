//! Provider of [`IteratorSimpleScanExt`].

use crate::{Diff, Trace, Trace2};

/// Extension Trait of [`Iterator`] for simple scan operations.
///
/// Provides more simplified versions of the [`scan`](Iterator::scan) method.
/// They are not as flexible as `scan`, but allows for shorter code in
/// certain cases.
pub trait IteratorSimpleScanExt: Iterator {
    /// Simplified version of the [`scan`](Iterator::scan) method.
    /// The process is focused solely on internal state tracking.
    ///
    /// The following is what differs from `scan`.
    ///
    /// * Iteration cannot be interrupted.
    /// * Internal state needs to be [`Clone`].
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

    /// An advanced version of the [`trace`](Self::trace) method.
    /// The resulting iterator tracks current state with previous state.
    ///
    /// The items in the result iterators are tuple, with the first element
    /// being the previous internal state, and the second element being the
    /// current internal state.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::simple_scan::IteratorSimpleScanExt;
    /// let result = (0..10).trace2(0, |s, x| s + x);
    /// let expect = (0..10).scan(0, |s, x| {
    ///     let prev = *s;
    ///     *s += x;
    ///     Some((prev, *s))
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
    ///   Alternate value of the previous element used only in the first iteration.
    /// * `f` -
    ///   Difference processing. The first argument is the current input item and
    ///   the second argument is the previous input item.
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
