use cxx::UniquePtr;

use crate::{alerts::TorrentState, ffi::ffi, torrent_handle::TorrentHandle};

/// Holds a snapshot of the status of a torrent, as queried by [`TorrentHandle::status()`].
#[derive(Debug)]
pub struct TorrentStatus {
    pub handle: TorrentHandle,
    pub state: TorrentState,
    pub progress: f64,
}

impl From<&ffi::torrent_status> for TorrentStatus {
    fn from(value: &ffi::torrent_status) -> Self {
        Self {
            handle: ffi::lt_torrent_status_handle(&value).into(),
            state: ffi::lt_torrent_status_state(&value).into(),
            progress: ffi::lt_torrent_status_progress(&value),
        }
    }
}

impl From<UniquePtr<ffi::torrent_status>> for TorrentStatus {
    fn from(value: UniquePtr<ffi::torrent_status>) -> Self {
        value.as_ref().unwrap().into()
    }
}
