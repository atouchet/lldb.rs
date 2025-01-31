// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, SBAddress, SBData, SBStream, SBTarget};
use std::ffi::CStr;
use std::fmt;

/// A machine instruction.
pub struct SBInstruction {
    /// The underlying raw `SBInstructionRef`.
    pub raw: sys::SBInstructionRef,
}

impl SBInstruction {
    /// Construct a new `Some(SBInstruction)` or `None`.
    pub fn maybe_wrap(raw: sys::SBInstructionRef) -> Option<SBInstruction> {
        if unsafe { sys::SBInstructionIsValid(raw) } {
            Some(SBInstruction { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBInstruction` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBInstructionIsValid(self.raw) }
    }

    /// Get the address of the instruction.
    pub fn address(&self) -> SBAddress {
        SBAddress::from(unsafe { sys::SBInstructionGetAddress(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn mnemonic(&self, target: &SBTarget) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBInstructionGetMnemonic(self.raw, target.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn operands(&self, target: &SBTarget) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBInstructionGetOperands(self.raw, target.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn comment(&self, target: &SBTarget) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBInstructionGetComment(self.raw, target.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn data(&self, target: &SBTarget) -> SBData {
        SBData::from(unsafe { sys::SBInstructionGetData(self.raw, target.raw) })
    }

    #[allow(missing_docs)]
    pub fn byte_size(&self) -> usize {
        unsafe { sys::SBInstructionGetByteSize(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_branch(&self) -> bool {
        unsafe { sys::SBInstructionDoesBranch(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn has_delay_slot(&self) -> bool {
        unsafe { sys::SBInstructionHasDelaySlot(self.raw) }
    }
}

impl Clone for SBInstruction {
    fn clone(&self) -> SBInstruction {
        SBInstruction {
            raw: unsafe { sys::CloneSBInstruction(self.raw) },
        }
    }
}

impl fmt::Debug for SBInstruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBInstructionGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBInstruction {{ {} }}", stream.data())
    }
}

impl Drop for SBInstruction {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBInstruction(self.raw) };
    }
}

impl From<sys::SBInstructionRef> for SBInstruction {
    fn from(raw: sys::SBInstructionRef) -> SBInstruction {
        SBInstruction { raw }
    }
}

unsafe impl Send for SBInstruction {}
unsafe impl Sync for SBInstruction {}
