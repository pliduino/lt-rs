use std::{fmt::Debug, marker::PhantomData};

use cxx::UniquePtr;

use crate::{ffi::ffi, info_hash::InfoHash};

// Owned type
pub struct AddTorrentParams(UniquePtr<ffi::add_torrent_params>);

pub struct AddTorrentParamsRef<'a>(*mut ffi::add_torrent_params, PhantomData<&'a ()>);

impl<'a> From<*mut ffi::add_torrent_params> for AddTorrentParamsRef<'a> {
    fn from(ptr: *mut ffi::add_torrent_params) -> AddTorrentParamsRef<'a> {
        AddTorrentParamsRef(ptr, PhantomData)
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for super::AddTorrentParams {}
    impl Sealed for &super::AddTorrentParams {}
    impl Sealed for super::AddTorrentParamsRef<'_> {}
}

pub trait AddTorrentParamsIntoPtr: private::Sealed {
    fn as_ptr(&self) -> *mut ffi::add_torrent_params;
}

impl AddTorrentParamsIntoPtr for AddTorrentParams {
    fn as_ptr(&self) -> *mut ffi::add_torrent_params {
        self.0.as_mut_ptr()
    }
}

impl AddTorrentParamsIntoPtr for &AddTorrentParams {
    fn as_ptr(&self) -> *mut ffi::add_torrent_params {
        self.0.as_mut_ptr()
    }
}

impl AddTorrentParamsIntoPtr for AddTorrentParamsRef<'_> {
    fn as_ptr(&self) -> *mut ffi::add_torrent_params {
        self.0
    }
}

impl Debug for AddTorrentParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddTorrentParams").finish()
    }
}

impl<'a> Debug for AddTorrentParamsRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddTorrentParamsRef").finish()
    }
}

impl AddTorrentParams {
    pub fn parse_magnet_uri(magnet_uri: &str) -> AddTorrentParams {
        AddTorrentParams(ffi::lt_parse_magnet_uri(magnet_uri))
    }

    pub fn load_resume_data(buf: &[u8]) -> AddTorrentParams {
        AddTorrentParams(unsafe { ffi::lt_read_resume_data(buf) })
    }
}

impl AddTorrentParams {
    pub fn set_path(&mut self, path: &str) {
        unsafe { ffi::lt_set_add_torrent_params_path(self.0.as_mut_ptr(), path) };
    }

    pub fn get_info_hash(&self) -> InfoHash {
        unsafe { ffi::lt_add_torrent_params_info_hash(self.0.as_mut_ptr()).into() }
    }

    pub fn write_resume_data_buf(&self) -> Vec<u8> {
        unsafe { ffi::lt_write_resume_data_buf(self.0.as_mut_ptr()) }
    }
}

impl<'a> AddTorrentParamsRef<'a> {
    pub fn make_owned(self) -> AddTorrentParams {
        unimplemented!()
    }

    pub fn set_path(&mut self, path: &str) {
        unsafe { ffi::lt_set_add_torrent_params_path(self.0, path) };
    }

    pub fn get_info_hash(&self) -> InfoHash {
        unsafe { ffi::lt_add_torrent_params_info_hash(self.0).into() }
    }

    pub fn write_resume_data_buf(&self) -> Vec<u8> {
        unsafe { ffi::lt_write_resume_data_buf(self.0) }
    }
}

impl From<UniquePtr<ffi::add_torrent_params>> for AddTorrentParams {
    fn from(inner: UniquePtr<ffi::add_torrent_params>) -> AddTorrentParams {
        AddTorrentParams(inner)
    }
}

// TODO: Check if this is safe
unsafe impl Send for AddTorrentParams {}
