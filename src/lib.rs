/*! Provider of [IteratorSimpleScanExt].

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*
*/

#![no_std]

mod diff;
mod iterator_simple_scan_ext;
mod msg;
mod trace;
mod trace2;

pub use diff::Diff;
pub use iterator_simple_scan_ext::IteratorSimpleScanExt;
pub use trace::Trace;
pub use trace2::Trace2;
