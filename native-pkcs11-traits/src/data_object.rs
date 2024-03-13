//! CKO_DATA object as defined in PKCS#11 2.40 4.5
//! [PKCS#11 2.40 ~ 4.5](http://docs.oasis-open.org/pkcs11/pkcs11-base/v2.40/os/pkcs11-base-v2.40-os.html#_Toc416959706)

use std::{any::Any, ffi::CString, hash::Hash};

pub trait DataObject: Send + Sync {
    /// The value of the object
    fn value(&self) -> Vec<u8>;

    /// The application that manages the object
    fn application(&self) -> CString;
    fn data_hash(&self) -> Vec<u8>;
    fn label(&self) -> String;
    fn delete(&self);
}

impl std::fmt::Debug for dyn DataObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data")
            .field("label", &self.label())
            .finish_non_exhaustive()
    }
}

impl PartialEq for dyn DataObject {
    fn eq(&self, other: &Self) -> bool {
        self.data_hash() == other.data_hash() && self.label() == other.label()
    }
}

impl Eq for dyn DataObject {}
impl Hash for dyn DataObject {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_id().hash(state);
        self.data_hash().hash(state);
        self.label().hash(state);
    }
}
