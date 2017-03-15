use ffi::h5o::{H5Oget_comment, H5Oset_comment};
use ffi::h5i::{H5Iget_name, H5Iget_file_id};
use ffi::h5f::H5Fget_name;

use error::Result;
use file::File;
use object::{Object, ObjectType, ObjectID};
use util::{get_h5_str, to_cstring};

use std::ptr;

/// A trait for HDF5 object types that can have a named location (file, group, dataset).
pub trait LocationType : ObjectType {}

impl<T: LocationType> Object<T> {
    /// Returns the name of the object within the file, or empty string if the object doesn't
    /// have a name (e.g., an anonymous dataset).
    pub fn name(&self) -> String {
        // TODO: should this return Result<String> or an empty string if it fails?
        h5lock!(get_h5_str(|m, s| { H5Iget_name(self.id(), m, s) }).unwrap_or("".to_string()))
    }

    /// Returns the name of the file containing the named object (or the file itself).
    pub fn filename(&self) -> String {
        // TODO: should this return Result<String> or an empty string if it fails?
        h5lock!(get_h5_str(|m, s| { H5Fget_name(self.id(), m, s) }).unwrap_or("".to_string()))
    }

    /// Returns a handle to the file containing the named object (or the file itself).
    pub fn file(&self) -> Result<File> {
        File::from_id(h5try!(H5Iget_file_id(self.id())))
    }

    /// Returns the commment attached to the named object, if any.
    pub fn comment(&self) -> Option<String> {
        // TODO: should this return Result<Option<String>> or fail silently?
        let comment = h5lock!(get_h5_str(|m, s| { H5Oget_comment(self.id(), m, s) }).ok());
        comment.and_then(|c| if c.is_empty() { None } else { Some(c) })
    }

    /// Set or the commment attached to the named object.
    pub fn set_comment(&self, comment: &str) -> Result<()> {
        let comment = to_cstring(comment)?;
        h5call!(H5Oset_comment(self.id(), comment.as_ptr())).and(Ok(()))
    }

    /// Clear the commment attached to the named object.
    pub fn clear_comment(&self) -> Result<()> {
        h5call!(H5Oset_comment(self.id(), ptr::null_mut())).and(Ok(()))
    }
}

#[cfg(test)]
pub mod tests {
    use file::File;
    use object::ObjectID;
    use test::{with_tmp_path, with_tmp_file};

    #[test]
    pub fn test_filename() {
        with_tmp_path(|path| {
            assert_eq!(File::open(&path, "w").unwrap().filename(), path.to_str().unwrap());
        })
    }

    #[test]
    pub fn test_name() {
        with_tmp_file(|file| {
            assert_eq!(file.name(), "/");
        })
    }

    #[test]
    pub fn test_file() {
        with_tmp_file(|file| {
            assert_eq!(file.file().unwrap().id(), file.id());
        })
    }

    #[test]
    pub fn test_comment() {
        with_tmp_file(|file| {
            assert!(file.comment().is_none());
            assert!(file.set_comment("foo").is_ok());
            assert_eq!(file.comment().unwrap(), "foo");
            assert!(file.clear_comment().is_ok());
            assert!(file.comment().is_none());
        })
    }
}
