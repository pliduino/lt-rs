use crate::alerts::UrlSeedAlert;
use crate::errors::LtrsError;
use crate::ffi::alerts::url_seed::ffi::{
    url_seed_alert_get_error, url_seed_alert_get_error_message, url_seed_alert_get_server_url,
};
use crate::torrent_handle::TorrentHandle;

impl UrlSeedAlert {
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
    pub fn server_url<'a>(&'a self) -> &'a str {
        unsafe { url_seed_alert_get_server_url(self.0) }
    }

    /// In case the web server sent an error message, this function returns it.
    #[inline(always)]
    pub fn error_message<'a>(&'a self) -> &'a str {
        unsafe { url_seed_alert_get_error_message(self.0) }
    }

    /// The error the web seed encountered. If this is not set, the server sent an error message, call error_message().
    #[inline(always)]
    pub fn error(&self) -> LtrsError {
        unsafe { url_seed_alert_get_error(self.0) }.into()
    }
}
