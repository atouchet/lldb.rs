// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{lldb_pid_t, sys, SBFileSpec};
use std::ffi::CStr;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct SBProcessInfo {
    /// The underlying raw `SBProcessInfoRef`.
    pub raw: sys::SBProcessInfoRef,
}

impl SBProcessInfo {
    #[allow(missing_docs)]
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBProcessInfoGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn executable_file(&self) -> SBFileSpec {
        SBFileSpec::from(unsafe { sys::SBProcessInfoGetExecutableFile(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn process_id(&self) -> lldb_pid_t {
        unsafe { sys::SBProcessInfoGetProcessID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn user_id(&self) -> Option<u32> {
        if unsafe { sys::SBProcessInfoUserIDIsValid(self.raw) } {
            Some(unsafe { sys::SBProcessInfoGetUserID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn group_id(&self) -> Option<u32> {
        if unsafe { sys::SBProcessInfoGroupIDIsValid(self.raw) } {
            Some(unsafe { sys::SBProcessInfoGetGroupID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn effective_user_id(&self) -> Option<u32> {
        if unsafe { sys::SBProcessInfoEffectiveUserIDIsValid(self.raw) } {
            Some(unsafe { sys::SBProcessInfoGetEffectiveUserID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn effective_group_id(&self) -> Option<u32> {
        if unsafe { sys::SBProcessInfoEffectiveGroupIDIsValid(self.raw) } {
            Some(unsafe { sys::SBProcessInfoGetEffectiveGroupID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn parent_process_id(&self) -> lldb_pid_t {
        unsafe { sys::SBProcessInfoGetParentProcessID(self.raw) }
    }
}

impl Clone for SBProcessInfo {
    fn clone(&self) -> SBProcessInfo {
        SBProcessInfo {
            raw: unsafe { sys::CloneSBProcessInfo(self.raw) },
        }
    }
}

impl Drop for SBProcessInfo {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBProcessInfo(self.raw) };
    }
}

impl From<sys::SBProcessInfoRef> for SBProcessInfo {
    fn from(raw: sys::SBProcessInfoRef) -> SBProcessInfo {
        SBProcessInfo { raw }
    }
}

unsafe impl Send for SBProcessInfo {}
unsafe impl Sync for SBProcessInfo {}

#[cfg(feature = "graphql")]
graphql_object!(SBProcessInfo: crate::SBDebugger | &self | {
    field name() -> &str {
        self.name()
    }

    field executable_file() -> SBFileSpec {
        self.executable_file()
    }

    // TODO(bm) This should be lldb_pid_t
    field process_id() -> i32 {
        self.process_id() as i32
    }

    // TODO(bm) This should be u32
    field user_id() -> i32 {
        self.user_id() as i32
    }

    // TODO(bm) This should be u32
    field group_id() -> i32 {
        self.group_id() as i32
    }

    field user_id_is_valid() -> bool {
        self.user_id_is_valid()
    }

    field group_id_is_valid() -> bool {
        self.group_id_is_valid()
    }

    // TODO(bm) This should be u32
    field effective_user_id() -> i32 {
        self.effective_user_id() as i32
    }

    // TODO(bm) This should be u32
    field effective_group_id() -> i32 {
        self.effective_group_id() as i32
    }

    field effective_user_id_is_valid() -> bool {
        self.effective_user_id_is_valid()
    }

    field effective_group_id_is_valid() -> bool {
        self.effective_group_id_is_valid()
    }

    // TODO(bm) This should be lldb_pid_t
    field parent_process_id() -> i32 {
        self.parent_process_id() as i32
    }
});
