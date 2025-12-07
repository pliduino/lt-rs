use crate::alerts::TorrentDeletedAlert;
use crate::ffi::alerts::torrent_deleted::ffi::torrent_deleted_alert_get_info_hashes;
use crate::info_hash::InfoHash;
use crate::torrent_handle::TorrentHandle;

impl TorrentDeletedAlert {
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
    pub fn info_hashes(&self) -> InfoHash {
        unsafe { torrent_deleted_alert_get_info_hashes(self.0) }.into()
    }
}
