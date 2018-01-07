//! Reference-counted smart pointer for Libaudioverse resources.
use std::ops::Deref;
use super::*;
use super::{libaudioverse_sys };
use check;

/// Manages reference counts for Libaudioverse handles. See the lifetime section of the libaudioverse manual for details. End users should not have to use this directly.
pub struct LavPtr {
    /// the underlying raw handle
    pub(crate) handle: libaudioverse_sys::LavHandle,
}

impl LavPtr {
    /// Creates a new reference counted pointer.
    pub fn new(handle : libaudioverse_sys::LavHandle) -> Result<LavPtr> {
        let mut is_first_access : i32 = 0;
        check(unsafe { libaudioverse_sys::Lav_handleGetAndClearFirstAccess(handle, &mut is_first_access) })?;
        
        // since reference counts start at 1, we only need to increment if this is not a newly created object
        if is_first_access == 0 {
            check(unsafe { libaudioverse_sys::Lav_handleIncRef(handle) })?;
        }
        
        Ok(LavPtr { handle })
    }
}

impl Clone for LavPtr {
    /// Increases the external reference count of the handle by 1.
    fn clone (&self) -> Self {
        LavPtr::new(self.handle).unwrap()
    }
}

impl Drop for LavPtr {
    /// Decreases the external reference count of the handle by 1.
    fn drop(&mut self) {
        unsafe { libaudioverse_sys::Lav_handleDecRef(self.handle) };
    }
}

impl Deref for LavPtr {
    type Target = libaudioverse_sys::LavHandle;
    
    /// Returns the underlying handle for convenience.
    fn deref(&self) -> &libaudioverse_sys::LavHandle {
        &(self.handle)
    }
}