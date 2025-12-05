use crate::{
    alerts::{ScrapeReplyAlert, protocol_version::ProtocolVersion},
    ffi::alerts::scrape_reply::ffi::{
        scrape_reply_alert_get_complete, scrape_reply_alert_get_incomplete,
        scrape_reply_alert_get_version,
    },
    torrent_handle::TorrentHandle,
};

impl ScrapeReplyAlert {
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
    pub fn version(&self) -> ProtocolVersion {
        ProtocolVersion::from_u8(unsafe { scrape_reply_alert_get_version(self.0) })
    }

    #[inline(always)]
    pub fn complete(&self) -> i32 {
        unsafe { scrape_reply_alert_get_complete(self.0) }
    }

    #[inline(always)]
    pub fn incomplete(&self) -> i32 {
        unsafe { scrape_reply_alert_get_incomplete(self.0) }
    }
}
