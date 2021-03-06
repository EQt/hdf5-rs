pub use self::H5I_type_t::*;

use libc::{c_int, c_uint, c_void, c_char, size_t, ssize_t};

use h5::{htri_t, hbool_t, herr_t, hsize_t};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum H5I_type_t {
    H5I_UNINIT      = -2,
    H5I_BADID       = -1,
    H5I_FILE        = 1,
    H5I_GROUP       = 2,
    H5I_DATATYPE    = 3,
    H5I_DATASPACE   = 4,
    H5I_DATASET     = 5,
    H5I_ATTR        = 6,
    H5I_REFERENCE   = 7,
    H5I_VFL         = 8,
    H5I_GENPROP_CLS = 9,
    H5I_GENPROP_LST = 10,
    H5I_ERROR_CLASS = 11,
    H5I_ERROR_MSG   = 12,
    H5I_ERROR_STACK = 13,
    H5I_NTYPES      = 14,
}

pub type hid_t = c_int;

pub const H5I_INVALID_HID: hid_t = -1;

pub type H5I_free_t = Option<extern fn(arg1: *mut c_void) -> herr_t>;
pub type H5I_search_func_t = Option<extern fn (obj: *mut c_void, id: hid_t, key: *mut c_void) ->
                                               c_int>;

extern {
    pub fn H5Iregister(_type: H5I_type_t, object: *const c_void) -> hid_t;
    pub fn H5Iobject_verify(id: hid_t, id_type: H5I_type_t) -> *mut c_void;
    pub fn H5Iremove_verify(id: hid_t, id_type: H5I_type_t) -> *mut c_void;
    pub fn H5Iget_type(id: hid_t) -> H5I_type_t;
    pub fn H5Iget_file_id(id: hid_t) -> hid_t;
    pub fn H5Iget_name(id: hid_t, name: *mut c_char, size: size_t) -> ssize_t;
    pub fn H5Iinc_ref(id: hid_t) -> c_int;
    pub fn H5Idec_ref(id: hid_t) -> c_int;
    pub fn H5Iget_ref(id: hid_t) -> c_int;
    pub fn H5Iregister_type(hash_size: size_t, reserved: c_uint, free_func: H5I_free_t) ->
                            H5I_type_t;
    pub fn H5Iclear_type(_type: H5I_type_t, force: hbool_t) -> herr_t;
    pub fn H5Idestroy_type(_type: H5I_type_t) -> herr_t;
    pub fn H5Iinc_type_ref(_type: H5I_type_t) -> c_int;
    pub fn H5Idec_type_ref(_type: H5I_type_t) -> c_int;
    pub fn H5Iget_type_ref(_type: H5I_type_t) -> c_int;
    pub fn H5Isearch(_type: H5I_type_t, func: H5I_search_func_t, key: *mut c_void) -> *mut c_void;
    pub fn H5Inmembers(_type: H5I_type_t, num_members: *mut hsize_t) -> herr_t;
    pub fn H5Itype_exists(_type: H5I_type_t) -> htri_t;
    pub fn H5Iis_valid(id: hid_t) -> htri_t;
}
