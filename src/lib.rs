/*! [`Iterator`] extensions for simple scan operation.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*
*/

#![no_std]
#![warn(missing_docs)]

pub mod prelude;

mod diff;
mod iterator_simple_scan_ext;
mod msg;
mod trace;
mod trace2;

pub use diff::*;
pub use iterator_simple_scan_ext::*;
pub use trace::*;
pub use trace2::*;
