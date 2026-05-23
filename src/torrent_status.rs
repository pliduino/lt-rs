use std::marker::PhantomData;

use cxx::{CxxVector, UniquePtr};

use crate::{
    alerts::TorrentState,
    ffi::ffi::*,
    torrent_handle::TorrentHandle,
};

/// Full torrent status snapshot. All fields captured in a single FFI call.
/// Returned by [`TorrentHandle::status()`].
pub struct TorrentStatus {
    pub download_rate:     i32,
    pub upload_rate:       i32,
    pub all_time_download: i64,
    pub all_time_upload:   i64,
    pub total_done:        i64,
    pub total_wanted:      i64,
    pub total_size:        i64,
    pub num_peers:         i32,
    pub num_seeds:         i32,
    pub num_complete:      i32,
    pub num_incomplete:    i32,
    pub progress:          f32,
    pub state:             TorrentState,
    pub is_seeding:        bool,
    pub is_finished:       bool,
    pub is_paused:         bool,
    /// Packed bitfield: bit N=1 means piece N is downloaded. Use `has_piece(n)`.
    pub pieces:            Vec<u8>,
    pub save_path:         String,
    pub name:              String,
}

impl TorrentStatus {
    pub(crate) fn from_inner(inner: UniquePtr<torrent_status>) -> Self {
        let snap = lt_torrent_status_snapshot(inner.as_ref().unwrap());
        Self::from_snapshot(snap)
    }

    pub(crate) fn from_snapshot(s: TorrentStatusSnapshot) -> Self {
        let state = TorrentState::from_u8(s.state);
        TorrentStatus {
            download_rate:     s.download_rate,
            upload_rate:       s.upload_rate,
            all_time_download: s.all_time_download,
            all_time_upload:   s.all_time_upload,
            total_done:        s.total_done,
            total_wanted:      s.total_wanted,
            total_size:        s.total_size,
            num_peers:         s.num_peers,
            num_seeds:         s.num_seeds,
            num_complete:      s.num_complete,
            num_incomplete:    s.num_incomplete,
            progress:          s.progress,
            state,
            is_seeding:        s.is_seeding,
            is_finished:       s.is_finished,
            is_paused:         s.is_paused,
            save_path:         s.save_path,
            name:              s.name,
            pieces:            s.pieces,
        }
    }

    /// Returns true if piece `idx` has been downloaded.
    pub fn has_piece(&self, idx: usize) -> bool {
        let byte = idx / 8;
        let bit  = idx % 8;
        self.pieces.get(byte).map(|&b| b & (1 << bit) != 0).unwrap_or(false)
    }

    /// Returns the count of downloaded pieces.
    pub fn num_pieces_done(&self) -> usize {
        self.pieces.iter().map(|b| b.count_ones() as usize).sum()
    }
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
    pub fn to_status(&self) -> TorrentStatus {
        let snap = lt_torrent_status_snapshot(unsafe { self.0.as_ref_unchecked() });
        TorrentStatus::from_snapshot(snap)
    }

    pub fn handle(&self) -> TorrentHandle {
        TorrentHandle::from_inner(unsafe { lt_torrent_status_handle(self.0) })
    }
}
