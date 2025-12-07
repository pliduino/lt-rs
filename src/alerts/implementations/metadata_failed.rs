use crate::alerts::MetadataFailedAlert;
use crate::errors::LtrsError;
use crate::ffi::alerts::metadata_failed::ffi::metadata_failed_alert_get_error;
use crate::torrent_handle::TorrentHandle;

impl MetadataFailedAlert {
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
    pub fn get_error(&self) -> LtrsError {
        unsafe { metadata_failed_alert_get_error(self.0) }.into()
    }
}
