use std::fmt::Debug;

use cxx::UniquePtr;

use crate::{ffi::ffi, info_hash::InfoHash};

pub struct AddTorrentParams {
    inner: UniquePtr<ffi::add_torrent_params>,
}

impl Debug for AddTorrentParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddTorrentParams").finish()
    }
}

impl AddTorrentParams {
    pub(super) fn inner(&self) -> &ffi::add_torrent_params {
        &self.inner
    }

    pub fn parse_magnet_uri(magnet_uri: &str) -> AddTorrentParams {
        AddTorrentParams {
            inner: ffi::lt_parse_magnet_uri(magnet_uri),
        }
    }

    pub fn set_path(&mut self, path: &str) {
        ffi::lt_set_add_torrent_params_path(self.inner.pin_mut(), path);
    }

    pub fn get_info_hash(&self) -> InfoHash {
        ffi::lt_add_torrent_params_info_hash(&self.inner).into()
    }

    pub fn write_resume_data_buf(&self) -> Vec<u8> {
        ffi::lt_write_resume_data_buf(&self.inner)
    }

    pub fn load_resume_data(buf: &[u8]) -> AddTorrentParams {
        AddTorrentParams {
            inner: ffi::lt_read_resume_data(buf),
        }
    }
}

impl From<UniquePtr<ffi::add_torrent_params>> for AddTorrentParams {
    fn from(inner: UniquePtr<ffi::add_torrent_params>) -> AddTorrentParams {
        AddTorrentParams { inner }
    }
}

// TODO: Check if this is safe
unsafe impl Send for AddTorrentParams {}
