// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, SBBroadcaster, SBStream};
use std::ffi::CStr;
use std::fmt;

/// An event.
pub struct SBEvent {
    /// The underlying raw `SBEventRef`.
    pub raw: sys::SBEventRef,
}

impl SBEvent {
    /// Construct a new `Some(SBEvent)` or `None`.
    pub fn maybe_wrap(raw: sys::SBEventRef) -> Option<SBEvent> {
        if unsafe { sys::SBEventIsValid(raw) } {
            Some(SBEvent { raw })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn new() -> SBEvent {
        Self::from(unsafe { sys::CreateSBEvent() })
    }

    /// Check whether or not this is a valid `SBEvent` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBEventIsValid(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn data_flavor(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBEventGetDataFlavor(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn event_type(&self) -> u32 {
        unsafe { sys::SBEventGetType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn broadcaster(&self) -> SBBroadcaster {
        SBBroadcaster::from(unsafe { sys::SBEventGetBroadcaster(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn broadcaster_class(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBEventGetBroadcasterClass(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn broadcaster_matches_ref(&self, broadcaster: &SBBroadcaster) -> bool {
        unsafe { sys::SBEventBroadcasterMatchesRef(self.raw, broadcaster.raw) }
    }
}

impl Clone for SBEvent {
    fn clone(&self) -> SBEvent {
        SBEvent {
            raw: unsafe { sys::CloneSBEvent(self.raw) },
        }
    }
}

impl fmt::Debug for SBEvent {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBEventGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBEvent {{ {} }}", stream.data())
    }
}

impl Drop for SBEvent {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBEvent(self.raw) };
    }
}

impl From<sys::SBEventRef> for SBEvent {
    fn from(raw: sys::SBEventRef) -> SBEvent {
        SBEvent { raw }
    }
}

unsafe impl Send for SBEvent {}
unsafe impl Sync for SBEvent {}
