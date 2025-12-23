use crate::alerts::HashFailedAlert;
use crate::ffi::alerts::hash_failed::ffi::hash_failed_alert_get_piece_index;
use crate::ffi::ffi::PieceIndex;
use crate::torrent_handle::TorrentHandle;

impl HashFailedAlert {
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
    pub fn piece_index(&self) -> PieceIndex {
        unsafe { hash_failed_alert_get_piece_index(self.0) }
    }
}
