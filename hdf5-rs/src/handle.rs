use ffi::h5i::{hid_t, H5I_type_t, H5Iget_type, H5Iis_valid, H5Iinc_ref, H5Idec_ref,
               H5I_INVALID_HID};
use ffi::h5i::H5I_type_t::*;

use error::Result;

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;

pub fn get_id_type(id: hid_t) -> H5I_type_t {
    h5lock!({
        let tp = h5lock!(H5Iget_type(id));
        let valid = id > 0 && tp > H5I_BADID && tp < H5I_NTYPES;
        if valid { tp } else { H5I_BADID }
    })
}

pub fn is_valid_id(id: hid_t) -> bool {
    h5lock!({
        let tp = get_id_type(id);
        tp > H5I_BADID && tp < H5I_NTYPES
    })
}

pub fn is_valid_user_id(id: hid_t) -> bool {
    h5lock!({
        H5Iis_valid(id) == 1
    })
}

struct Registry {
    registry: Mutex<HashMap<hid_t, Arc<RwLock<hid_t>>>>,
}

impl Default for Registry {
    fn default() -> Registry {
        Registry::new()
    }
}

impl Registry {
    pub fn new() -> Registry {
        Registry { registry: Mutex::new(HashMap::new()) }
    }

    pub fn new_handle(&self, id: hid_t) -> Arc<RwLock<hid_t>> {
        let mut registry = self.registry.lock().unwrap();
        let handle = registry.entry(id).or_insert_with(|| Arc::new(RwLock::new(id)));
        if *handle.read().unwrap() != id {
            // an id may be left dangling by previous invalidation of a linked handle
            *handle = Arc::new(RwLock::new(id));
        }
        handle.clone()
    }
}

pub struct Handle {
    id: Arc<RwLock<hid_t>>,
}

impl Handle {
    pub fn new(id: hid_t) -> Result<Handle> {
        lazy_static! {
            static ref REGISTRY: Registry = Registry::new();
        }
        h5lock!({
            if is_valid_user_id(id) {
                Ok(Handle{ id: REGISTRY.new_handle(id) })
            } else {
                Err(From::from(format!("Invalid handle id: {}", id)))
            }
        })
    }

    pub fn invalid() -> Handle {
        Handle { id: Arc::new(RwLock::new(H5I_INVALID_HID)) }
    }

    pub fn id(&self) -> hid_t {
        *self.id.read().unwrap()
    }

    pub fn invalidate(&self) {
        *self.id.write().unwrap() = H5I_INVALID_HID;
    }

    pub fn incref(&self) {
        if is_valid_user_id(self.id()) {
            h5lock!(H5Iinc_ref(self.id()));
        }
    }

    pub fn decref(&self) {
        h5lock!({
            if is_valid_user_id(self.id()) {
                H5Idec_ref(self.id());
            }
            // must invalidate all linked IDs because the library reuses them internally
            if !is_valid_user_id(self.id()) && !is_valid_id(self.id()) {
                self.invalidate();
            }
        })
    }

    pub fn is_valid(&self) -> bool {
        is_valid_user_id(self.id())
    }
}

impl Clone for Handle {
    fn clone(&self) -> Handle {
        h5lock!({
            self.incref();
            Handle::new(self.id()).unwrap_or(Handle::invalid())
        })
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        h5lock!(self.decref());
    }
}
