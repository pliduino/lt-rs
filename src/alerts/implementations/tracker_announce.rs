use crate::alerts::TrackerAnnounceAlert;
use crate::ffi::alerts::tracker_announce::ffi::{
    tracker_announce_alert_get_event, tracker_announce_alert_get_version,
};
use crate::torrent_handle::TorrentHandle;

impl TrackerAnnounceAlert {
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
    pub fn event(&self) -> u8 {
        unsafe { tracker_announce_alert_get_event(self.0) }
    }

    #[inline(always)]
    pub fn version(&self) -> u8 {
        unsafe { tracker_announce_alert_get_version(self.0) }
    }
}
