use std::marker::PhantomData;

use cxx::CxxVector;

use crate::{
    alerts::TorrentState,
    ffi::ffi::{
        lt_torrent_status_handle, lt_torrent_status_progress, lt_torrent_status_state,
        torrent_status,
    },
    torrent_handle::TorrentHandle,
};

/// Holds a snapshot of the status of a torrent, as queried by [`TorrentHandle::status()`].
pub struct TorrentStatus {
    pub handle: TorrentHandle,
    pub state: TorrentState,
    pub progress: f64,
}

/// Reference to C++ vector
pub struct TorrentStatusVecRef<'a>(*mut CxxVector<torrent_status>, PhantomData<&'a ()>);

impl<'a> From<*mut CxxVector<torrent_status>> for TorrentStatusVecRef<'a> {
    fn from(ptr: *mut CxxVector<torrent_status>) -> Self {
        Self(ptr, PhantomData)
    }
}

impl<'a> TorrentStatusVecRef<'a> {
    pub fn iter(&self) -> impl Iterator<Item = TorrentStatusRef<'a>> {
        unsafe {
            let v: &'a CxxVector<torrent_status> = &*self.0;
            v.iter().map(|s| s.into())
        }
    }
}

pub struct TorrentStatusRef<'a>(*mut torrent_status, PhantomData<&'a ()>);

impl<'a> From<*mut torrent_status> for TorrentStatusRef<'a> {
    fn from(ptr: *mut torrent_status) -> Self {
        Self(ptr, PhantomData)
    }
}

impl<'a> From<&'a torrent_status> for TorrentStatusRef<'a> {
    fn from(ptr: &'a torrent_status) -> Self {
        Self(
            ptr as *const torrent_status as *mut torrent_status,
            PhantomData,
        )
    }
}

impl<'a> TorrentStatusRef<'a> {
    pub fn handle(&self) -> TorrentHandle {
        TorrentHandle::from_inner(unsafe { lt_torrent_status_handle(self.0) })
    }

    pub fn state(&self) -> TorrentState {
        let state = unsafe { lt_torrent_status_state(self.0) };
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                TorrentState::from(state)
            } else {
                unsafe { std::mem::transmute(state) }
            }
        }
    }

    pub fn progress(&self) -> f64 {
        unsafe { lt_torrent_status_progress(self.0) }
    }
}
