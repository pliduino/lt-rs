use crate::alerts::StorageMovedAlert;
use crate::ffi::alerts::storage_moved::ffi::{
    storage_moved_alert_get_old_path, storage_moved_alert_get_storage_path,
};
use crate::torrent_handle::TorrentHandle;

impl StorageMovedAlert {
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

    #[inline(always)]
    pub fn storage_path<'a>(&'a self) -> &'a str {
        unsafe { storage_moved_alert_get_storage_path(self.0) }
    }

    #[inline(always)]
    pub fn old_path<'a>(&'a self) -> &'a str {
        unsafe { storage_moved_alert_get_old_path(self.0) }
    }
}
