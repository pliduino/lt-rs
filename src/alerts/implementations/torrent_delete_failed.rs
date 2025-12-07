use crate::alerts::TorrentDeleteFailedAlert;
use crate::errors::LtrsError;
use crate::ffi::alerts::torrent_delete_failed::ffi::{
    torrent_delete_failed_alert_get_error, torrent_delete_failed_alert_get_info_hashes,
};
use crate::info_hash::InfoHash;
use crate::torrent_handle::TorrentHandle;

impl TorrentDeleteFailedAlert {
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
    pub fn error(&self) -> LtrsError {
        unsafe { torrent_delete_failed_alert_get_error(self.0) }.into()
    }

    #[inline(always)]
    pub fn info_hashes(&self) -> InfoHash {
        unsafe { torrent_delete_failed_alert_get_info_hashes(self.0) }.into()
    }
}
