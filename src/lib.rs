#![cfg_attr(feature = "lint", feature(plugin))]
#![cfg_attr(feature = "lint", plugin(clippy))]
#![cfg_attr(feature = "lint", allow(block_in_if_condition_stmt, needless_return))]
#![cfg_attr(not(test), allow(dead_code))]

pub use container::Container;
pub use dataset::Dataset;
pub use datatype::{Datatype, ToDatatype};
pub use error::{Result, Error};
pub use file::File;
pub use filters::Filters;
pub use group::Group;
pub use location::Location;
pub use object::Object;
pub use space::{Dimension, Ix, Dataspace};

extern crate libc;
extern crate num;

extern crate libhdf5_lib as lib;
extern crate libhdf5_sys as ffi;
extern crate remutex;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate tempdir;

#[cfg(test)]
extern crate regex;

#[macro_use]
mod macros;

mod container;
mod dataset;
mod datatype;
mod error;
mod file;
mod filters;
mod group;
mod handle;
mod location;
mod object;
mod plist;
mod space;
mod sync;
mod util;

#[allow(dead_code)]
mod globals;

pub mod prelude;

#[cfg(test)]
pub mod test;

/// Returns the version of the HDF5 library that the crate was compiled against.
pub fn hdf5_version() -> (u8, u8, u8) {
    lib::hdf5_version().unwrap_or((0, 0, 0))
}

#[cfg(test)]
pub mod tests {
    use super::hdf5_version;

    #[test]
    pub fn test_hdf5_version() {
        assert!(hdf5_version() >= (1, 8, 0));
    }
}
