use crate::{
    alerts::{ScrapeFailedAlert, protocol_version::ProtocolVersion},
    errors::LtrsError,
    ffi::alerts::scrape_failed::ffi::{
        scrape_failed_alert_get_error, scrape_failed_alert_get_error_message,
        scrape_failed_alert_get_version,
    },
    torrent_handle::TorrentHandle,
};

impl ScrapeFailedAlert {
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
    pub fn tracker_url<'a>(&'a self) -> &'a str {
        self.as_tracker_alert().tracker_url()
    }

    #[inline(always)]
    pub fn local_endpoint(&self) {
        self.as_tracker_alert().local_endpoint()
    }

    #[inline(always)]
    pub fn error_message<'a>(&'a self) -> &'a str {
        unsafe { scrape_failed_alert_get_error_message(self.0) }
    }

    #[inline(always)]
    pub fn error(&self) -> LtrsError {
        unsafe { scrape_failed_alert_get_error(self.0) }.into()
    }

    #[inline(always)]
    pub fn version(&self) -> ProtocolVersion {
        ProtocolVersion::from_u8(unsafe { scrape_failed_alert_get_version(self.0) })
    }
}
