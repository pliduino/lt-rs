use crate::{
    alerts::TorrentErrorAlert, errors::LtrsError,
    ffi::alerts::torrent_error::ffi::torrent_error_alert_get_error,
};

impl TorrentErrorAlert {
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
        unsafe { torrent_error_alert_get_error(self) }.into();
    }

    #[inline(always)]
    pub fn filename<'a>(&'a self) -> &'a str {
        unsafe { torrent_error_alert_get_filename(self) }
    }
}
