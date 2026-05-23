use cxx::UniquePtr;
use crate::ffi::ffi::{IpFilterWrapper, lt_ip_filter_new, lt_ip_filter_add_rule, lt_ip_filter_access};

pub struct IpFilter(UniquePtr<IpFilterWrapper>);

pub mod access_flags {
    pub const BLOCKED: u32 = 1;
}

impl IpFilter {
    pub fn new() -> Self {
        IpFilter(lt_ip_filter_new())
    }

    /// Block or allow an IP range [start, end] (inclusive, dotted-decimal strings).
    /// `flags = access_flags::BLOCKED` to block, `0` to allow.
    pub fn add_rule(&mut self, start: &str, end: &str, flags: u32) {
        lt_ip_filter_add_rule(self.0.pin_mut(), start, end, flags);
    }

    /// Returns `access_flags::BLOCKED` if the address is blocked, `0` otherwise.
    pub fn access(&self, addr: &str) -> u32 {
        lt_ip_filter_access(&self.0, addr)
    }

    pub(crate) fn inner(&self) -> &IpFilterWrapper {
        self.0.as_ref().unwrap()
    }

    pub(crate) fn from_inner(inner: UniquePtr<IpFilterWrapper>) -> Self {
        IpFilter(inner)
    }
}

impl Default for IpFilter { fn default() -> Self { Self::new() } }
unsafe impl Send for IpFilter {}
