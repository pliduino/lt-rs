use crate::alerts::StorageMovedFailedAlert;
use crate::alerts::operation::Operation;
use crate::errors::LtrsError;
use crate::ffi::alerts::storage_moved_failed::ffi::{
    storage_moved_failed_alert_file_path, storage_moved_failed_alert_get_error,
    storage_moved_failed_alert_get_op,
};
use crate::torrent_handle::TorrentHandle;

impl StorageMovedFailedAlert {
    #[inline(always)]
    pub fn handle(&self) -> TorrentHandle {
        self.as_torrent_alert().handle()
    }

    #[inline(always)]
    pub fn torrent_name<'a>(&'a self) -> &'a str {
        self.as_torrent_alert().torrent_name()
    }

    #[inline(always)]
    pub fn message(&self) -> String {
        self.as_torrent_alert().message()
    }

    /// If the error happened for a specific file, this returns its path.
    #[inline(always)]
    pub fn file_path<'a>(&'a self) -> &'a str {
        unsafe { storage_moved_failed_alert_file_path(self.0) }
    }

    #[inline(always)]
    pub fn error(&self) -> LtrsError {
        unsafe { storage_moved_failed_alert_get_error(self.0) }.into()
    }

    /// This indicates what underlying operation caused the error
    #[inline(always)]
    pub fn op(&self) -> Operation {
        unsafe { storage_moved_failed_alert_get_op(self.0) }.into()
    }
}
