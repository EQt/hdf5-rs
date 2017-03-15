//! The HDF5 prelude module.
//!
//! The purpose of this module is to provide reexports of many core `hdf5` traits so that
//! they can be then glob-imported all at once:
//!
//! ```ignore
//! use h5::prelude::*;
//! ```
//! This module provides reexports of such traits as `Object`, `Location` and `Container` and
//! does not expose any structures or functions.


pub use super::Dimension;
pub use super::ToDatatype;

// A few extra traits that are not reexported on the crate level.
pub use datatype::{AnyDatatype, AtomicDatatype};
